use photo::{Sample, Scene, Settings, Tile};

fn main() {
    let settings_filepath = read_command_line_arguments();
    let settings = load_settings(&settings_filepath);
    println!("-- Settings --\n{}", settings);

    let scene = Scene::new();
    println!("-- Scene --\n{}", scene);

    // render_image_in_tiles(&scene, &settings);
    // println!("Done!");
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

fn _render_image_in_tiles(scene: &Scene, settings: &Settings) {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let total_num_tiles = settings.total_num_tiles();
    let num_x_tile = settings.num_tiles()[0];
    // (0..total_num_tiles).into_par_iter().for_each(|n| {
    (0..total_num_tiles).into_iter().for_each(|n| {
        let x = n % num_x_tile;
        let y = n / num_x_tile;
        let _tile = _render_tile(scene, settings, [x, y]);
    });
}

fn _render_tile(scene: &Scene, settings: &Settings, tile_index: [usize; 2]) -> Tile {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let tile = Tile::new(tile_index, settings.tile_resolution());
    // data.par_mapv_inplace(|sample| run_sample(settings, scene, tile_index, sample));
    tile
}

fn _run_sample(
    settings: &Settings,
    scene: &Scene,
    offset: [usize; 2],
    mut sample: Sample,
) -> Sample {
    debug_assert!(scene.is_valid());
    debug_assert!(settings.is_valid());

    let x = sample.index.0 + offset[0] * settings.tile_resolution[0];
    let y = sample.index.1 + offset[1] * settings.tile_resolution[1];
    println!("x: {}, y: {}", x, y);
    sample.total += 1.0;
    sample
}
