use photo::{Sample, Scene, Settings, Tile};

fn main() {
    let settings_filepath = read_command_line_arguments();
    let settings = load_settings(&settings_filepath);
    println!("-- Settings -----------------------\n{}", settings);

    let scene = Scene::new();
    println!("-- Scene --------------------------\n{}", scene);

    render_image_in_tiles(&scene, &settings);
    println!("-- Complete -----------------------");
}

/// Read the command line arguments.
/// Specifically, read the path to the settings file.
fn read_command_line_arguments() -> std::path::PathBuf {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <settings.yaml>", args[0]);
        std::process::exit(1);
    }

    std::path::PathBuf::from(&args[1])
}

/// Load the settings from the given file.
fn load_settings(settings_filepath: &std::path::Path) -> Settings {
    let file_string =
        std::fs::read_to_string(settings_filepath).expect("Unable to read settings file");

    let settings: Settings =
        serde_yaml::from_str(&file_string).expect("Unable to parse settings file");

    if !settings.is_valid() {
        println!("Invalid settings file: {}", settings_filepath.display());
        std::process::exit(1);
    }

    settings
}

/// Render the image in an array of tiles.
fn render_image_in_tiles(scene: &Scene, settings: &Settings) {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let total_num_tiles = settings.total_num_tiles();
    let num_x_tile = settings.num_tiles()[0];

    let pb = indicatif::ProgressBar::new(total_num_tiles as u64);
    pb.inc(0);
    (0..total_num_tiles).into_iter().for_each(|n| {
        let x = n % num_x_tile;
        let y = n / num_x_tile;
        let tile = render_tile(scene, settings, [x, y]);
        tile.save();
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

    if settings.display_tiles {
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

    let pixel_x = sample.index.0 + (tile_index[0] * settings.tile_resolution[0]);
    let pixel_y = sample.index.1 + (tile_index[1] * settings.tile_resolution[1]);

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
