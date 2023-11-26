use std::path::Path;

use crate::{
    render::Tile,
    world::{Camera, Scene},
};

pub fn render(output_directory: &Path, scene: &Scene, camera: &Camera) {
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
        [0.0, 0.0, 0.0, 1.0].into(),
    );

    tile.data.par_mapv_inplace(|sample| {
        let ray = camera.generate_ray(sample.pixel_index, [0, 0]);
        scene.sample(sample.pixel_index, ray)
    });

    tile
}
