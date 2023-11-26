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

    (0..total_num_tiles).into_iter().for_each(|n| {
        let row = n % rows;
        let column = n / rows;
        let tile = render_tile(scene, camera, [row, column]);
        let file_name = output_directory.join(format!("tile_{:03}_{:03}.png", row, column));
        tile.save(&file_name);
    });
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
