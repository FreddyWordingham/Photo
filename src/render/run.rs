//! Orchestrates the rendering of photographs imaging a scene.

use crate::{
    render::{Settings, Tile},
    world::{Camera, Scene},
};

/// Render all the [`Tile`]s of a photograph.
pub fn render_tiles<'a>(
    settings: &'a Settings,
    scene: &'a Scene,
    camera: &'a Camera,
) -> impl Iterator<Item = Tile> + 'a {
    let [rows, columns] = camera.num_tiles();
    let total_num_tiles = rows * columns;

    (0..total_num_tiles).map(move |n| {
        let row = n % rows;
        let column = n / rows;
        let tile_index = [row, column];
        render_tile(settings, scene, camera, tile_index)
    })
}

/// Render an individual [`Tile`] of a photograph.
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

    tile.samples.par_mapv_inplace(|mut sample| {
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
