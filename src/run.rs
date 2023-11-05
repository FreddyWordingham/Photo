use crate::{
    render::{Sample, Tile},
    world::Scene,
    Settings,
};

/// Render the image in an array of tiles.
pub fn render_image_in_tiles(
    scene: &Scene,
    settings: &Settings,
    output_directory: &std::path::Path,
) {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let total_num_tiles = settings.total_num_tiles();
    let num_tile_rows = settings.num_tiles()[0];

    let pb = indicatif::ProgressBar::new(total_num_tiles as u64);
    pb.inc(0);
    (0..total_num_tiles).into_iter().for_each(|n| {
        let row = n % num_tile_rows;
        let column = n / num_tile_rows;
        let tile = render_tile(scene, settings, [row, column]);
        tile.save(output_directory);
        pb.inc(1);
    });
}

/// Render a single tile.
fn render_tile(scene: &Scene, settings: &Settings, tile_index: [usize; 2]) -> Tile {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let mut tile = Tile::new(tile_index, settings.tile_resolution());
    tile.data
        .par_mapv_inplace(|sample| run_sample(settings, scene, tile_index, sample));

    if settings.display_in_terminal {
        println!("{}", tile);
    }

    tile
}

/// Run a single pixel sample.
fn run_sample(
    settings: &Settings,
    scene: &Scene,
    tile_index: [usize; 2],
    mut sample: Sample,
) -> Sample {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let pixel_x = sample.sample_index[1] + (tile_index[1] * settings.tile_resolution[1]);
    let pixel_y = sample.sample_index[0] + (tile_index[0] * settings.tile_resolution[0]);

    if pixel_x == pixel_y {
        sample.colour.blue = 1.0;
        sample.colour.alpha = 1.0;
    } else if pixel_x == (2 * pixel_y) {
        sample.colour.green = 1.0;
        sample.colour.alpha = 1.0;
    } else {
        sample.colour.red = 0.1;
        sample.colour.green = 0.1;
        sample.colour.blue = 0.1;
        sample.colour.alpha = 1.0;
    }

    sample
}
