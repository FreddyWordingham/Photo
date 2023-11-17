use indicatif::ProgressBar;
use palette::Srgba;
use std::{fs::create_dir_all, path::Path};

use crate::{
    engine,
    input::Settings,
    render::Sample,
    render::Tile,
    utility::terminal,
    world::{Camera, Scene},
};

/// Render the all the cameras.
pub fn render_all_cameras(settings: &Settings, scene: &Scene, output_directory: &Path) {
    debug_assert!(settings.is_valid());

    for (camera_name, camera_settings) in settings.cameras() {
        println!("{}", terminal::subheading(camera_name));
        let camera = camera_settings.build();
        println!("{}", camera);

        // Create output directory for camera.
        let camera_output_directory = output_directory.join(camera_name);
        if !camera_output_directory.exists() {
            create_dir_all(&camera_output_directory).unwrap();
        }

        render_camera(settings, scene, &camera, &camera_output_directory);
    }
}

/// Render a camera's image.
pub fn render_camera(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    camera_output_directory: &Path,
) {
    let [num_tile_rows, num_tile_columns] = camera.image_resolution();
    let total_num_tiles = num_tile_rows * num_tile_columns;

    let pb = ProgressBar::new((total_num_tiles) as u64);
    pb.inc(0);
    (0..total_num_tiles).into_iter().for_each(|n| {
        let row = n % num_tile_rows;
        let column = n / num_tile_rows;
        let tile = render_tile(settings, scene, camera, [row, column]);
        tile.save(camera_output_directory);
        pb.inc(1);
    });
    pb.finish();
}

/// Render a single tile.
fn render_tile(
    settings: &Settings,
    scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
) -> Tile {
    let mut tile = Tile::new(tile_index, camera.tile_resolution());
    tile.data
        .par_mapv_inplace(|sample| super_sample_pixel(settings, scene, camera, tile_index, sample));

    if settings.print_tiles_to_terminal() {
        println!("{}", tile);
    }

    tile
}

/// Super sample a single pixel.
fn super_sample_pixel(
    settings: &Settings,
    _scene: &Scene,
    camera: &Camera,
    tile_index: [usize; 2],
    mut sample: Sample,
) -> Sample {
    debug_assert!(settings.is_valid());

    let row = sample.sample_index[0] + (tile_index[0] * camera.tile_resolution()[0]);
    let column = sample.sample_index[1] + (tile_index[1] * camera.tile_resolution()[1]);

    let ss = camera.super_samples_per_axis();
    let inv_ss = 1.0 / ss as f64;

    let mut colours = Vec::with_capacity(ss * ss);
    for xi in 0..ss {
        let dx = (xi as f64 + 0.5) * inv_ss;
        let px = row as f64 + dx;
        for yi in 0..camera.super_samples_per_axis() {
            let dy = (yi as f64 + 0.5) * inv_ss;
            let py = column as f64 + dy;
            let ray = camera.gen_ray(px, py);
            colours.push(engine::sample_scene(_scene, ray));
        }
    }
    sample.colour = average_color(colours);

    sample
}

/// Calculate the average colour of a list of colours.
fn average_color(colors: Vec<Srgba>) -> Srgba {
    let mut sum = Srgba::new(0.0, 0.0, 0.0, 0.0);
    let count = colors.len() as f32;

    for color in colors {
        sum.red += color.red;
        sum.green += color.green;
        sum.blue += color.blue;
        sum.alpha += color.alpha;
    }

    Srgba::new(
        sum.red / count,
        sum.green / count,
        sum.blue / count,
        sum.alpha / count,
    )
}
