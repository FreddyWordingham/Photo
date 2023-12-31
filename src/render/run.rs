//! Orchestrates the rendering of photographs imaging a scene.

use std::{fs::create_dir_all, io::Error};

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    render::{Settings, Tile},
    world::{Camera, Scene},
};

/// Render an photograph use multiple threads.
///
/// # Errors
///
/// Returns a [`Box<dyn Error>`] if the output directory cannot be created,
/// or if an error occurs while rendering.
///
/// # Panics
///
/// Panics a [`Tile`] cannot be saved.
#[inline]
#[allow(clippy::expect_used, clippy::integer_division)]
pub fn parallel(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    image_name: &str,
) -> Result<(), Error> {
    let output_directory = settings.output_directory.join(image_name);
    create_dir_all(&output_directory)?;

    let [rows, columns] = camera.num_tiles();
    let total_num_tiles = rows * columns;

    let pb = ProgressBar::new(total_num_tiles as u64).with_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
            .expect("Failed to set progress-bar style.")
            .progress_chars("\\/"),
    );
    (0..total_num_tiles).for_each(|n| {
        pb.inc(1);
        let row = n % rows;
        let column = n / rows;
        let file_name = output_directory.join(format!("tile_{row:03}_{column:03}.png"));

        let tile = render_tile(settings, scene, camera, [row, column]);
        tile.save(&file_name).expect("Failed to save tile.");
    });
    pb.finish_with_message(format!(
        "Completed rendering {}",
        output_directory.display()
    ));

    Ok(())
}

/// Render a tile of a photograph.
#[must_use]
#[inline]
fn render_tile(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
) -> Tile {
    let mut tile = Tile::new(tile_index, camera.tile_resolution());

    let engine = camera.engine();
    let super_samples_per_axis = camera.super_samples_per_axis();
    let inv_total_super_samples = 1.0 / (super_samples_per_axis * super_samples_per_axis) as f32;

    // tile.samples.par_mapv_inplace(|mut sample| {
    tile.samples.mapv_inplace(|mut sample| {
        let start_time = std::time::Instant::now();
        for xi in 0..super_samples_per_axis {
            for yi in 0..super_samples_per_axis {
                let ray = camera.generate_ray(sample.pixel_index, [xi, yi]);
                sample.colour += engine(settings, scene, ray);
            }
        }
        sample.time = start_time.elapsed().as_nanos();
        sample *= inv_total_super_samples;
        sample
    });

    tile
}
