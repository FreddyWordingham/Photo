use std::collections::HashMap;

use photo::{
    input::{CameraSettings, LightingSettings, SceneSettings, Settings},
    utility::setup,
};

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }};
}

fn main() {
    let settings_filepath = setup::read_command_line_arguments();

    let print_tiles_to_terminal = false;
    let resolution = [1080, 1920]; // [rows, columns]
    let tile_resolution = [108, 192]; // [rows, columns

    let cameras = hashmap!(
        "camera 0".to_string() => CameraSettings {
            position: [10.0, 5.0, 7.0], // [x, y, z]
            target: [0.0, 0.0, 0.0],   // [x, y, z]
            field_of_view: 90.0,       // [degrees]
            resolution,                // [rows, columns]
            tile_resolution,           // [rows, columns]
        }
    );

    let scene = SceneSettings {};

    let sun_position = [-40.0, 70.0, 100.0]; // [x, y, z]
    let lighting = LightingSettings::new(sun_position);

    let settings = Settings::new(print_tiles_to_terminal, scene, lighting, cameras);

    if !settings.is_valid() {
        panic!("ERROR! Refusal to generate settings file due to invalid settings.");
    }

    println!("Generating settings file...");
    settings.save(&settings_filepath);
}