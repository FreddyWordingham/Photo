use core::panic;

use photo::{
    input::{CameraSettings, Settings},
    utility::setup,
};

fn main() {
    let settings_filepath = setup::read_command_line_arguments();

    let resolution = [1080, 1920]; // [rows, columns]
    let tile_resolution = [108, 192]; // [rows, columns
    let cameras = vec![CameraSettings {
        name: "Camera 1".to_string(), // String
        position: [0.0, 0.0, 0.0],    // [x, y, z]
        target: [0.0, 0.0, 0.0],      // [x, y, z]
        field_of_view: 90.0,          // [degrees]
    }];

    let settings = Settings::new(resolution, tile_resolution, cameras);

    if !settings.is_valid() {
        panic!("ERROR! Refusal to generate settings file due to invalid settings.");
    }

    println!("Generating settings file...");
    setup::save_settings(&settings, &settings_filepath);
}
