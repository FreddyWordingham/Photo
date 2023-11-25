use std::path::Path;

use crate::{
    render::{Sample, Tile},
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

fn render_tile(_scene: &Scene, camera: &Camera, tile_index: [usize; 2]) -> Tile {
    let [rows, columns] = camera.tile_resolution();
    let total_num_pixels = rows * columns;
    let row_offset = tile_index[0] * rows;
    let column_offset = tile_index[1] * columns;

    let mut tile = Tile::new([rows, columns]);

    (0..total_num_pixels).into_iter().for_each(|n| {
        let row = n % rows;
        let column = n / rows;

        if (row_offset + row) == (column_offset + column) {
            let sample = Sample::new([0.0, 0.0, 1.0, 1.0].into());
            tile.data[[row, column]] = sample;
        }
    });

    tile
}
