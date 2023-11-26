use indicatif::{ProgressBar, ProgressStyle};
use std::fs::create_dir_all;

use crate::{
    render::{Settings, Tile},
    world::{Camera, Scene},
};

pub fn render(settings: &Settings, scene: &Scene, camera_id: &str, camera: &Camera) {
    let output_directory = settings.output_directory().join(camera_id);
    create_dir_all(&output_directory).expect("Unable to create camera output directory");

    let [rows, columns] = camera.num_tiles();
    let total_num_tiles = rows * columns;

    let pb = ProgressBar::new(total_num_tiles as u64).with_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.green/red}] [{pos}/{len}] {percent}% ({eta}) {msg}")
                .expect("Failed to set progress-bar style.")
                .progress_chars("\\/"),
        );
    (0..total_num_tiles).into_iter().for_each(|n| {
        pb.inc(1);
        let row = n % rows;
        let column = n / rows;
        let tile = render_tile(scene, camera, [row, column]);
        let file_name = output_directory.join(format!("tile_{:03}_{:03}.png", row, column));
        tile.save(&file_name);
    });
    pb.finish_with_message(format!("Rendered {}", camera_id));
}

fn render_tile(scene: &Scene, camera: &Camera, tile_index: [usize; 2]) -> Tile {
    let mut tile = Tile::new(
        tile_index,
        camera.tile_resolution(),
        [0.0, 0.0, 0.0, 0.0].into(),
    );

    tile.data.par_mapv_inplace(|sample| {
        let mut sample = sample;
        for xi in 0..camera.super_samples_per_axis() {
            for yi in 0..camera.super_samples_per_axis() {
                let ray = camera.generate_ray(sample.pixel_index, [xi, yi]);
                sample += scene.sample(sample.pixel_index, ray);
            }
        }
        sample /= (camera.super_samples_per_axis() * camera.super_samples_per_axis()) as f32;
        sample
    });

    tile
}
