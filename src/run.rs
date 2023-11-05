use crate::{
    input::Settings,
    render::{Sample, Tile},
    world::Scene,
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
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
    pb.finish_with_message("Render complete");
}

/// Render a single tile.
fn render_tile(scene: &Scene, settings: &Settings, tile_index: [usize; 2]) -> Tile {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let mut tile = Tile::new(tile_index, settings.tile_resolution());
    tile.data
        .par_mapv_inplace(|sample| run_sample(settings, scene, tile_index, sample));

    println!("{}", tile);

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

    let row = sample.sample_index[0] + (tile_index[0] * settings.tile_resolution()[0]);
    let column = sample.sample_index[1] + (tile_index[1] * settings.tile_resolution()[1]);

    if row == column {
        sample.colour.blue = 1.0;
        sample.colour.alpha = 1.0;
    } else if row == (2 * column) {
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
