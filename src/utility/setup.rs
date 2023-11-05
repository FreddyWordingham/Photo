use std::{
    env::args,
    fs::{create_dir, read_to_string, write},
    path::{Path, PathBuf},
    process::exit,
};

use crate::input::Settings;

/// Read the command line arguments.
/// Specifically, read the path to the settings file.
pub fn read_command_line_arguments() -> PathBuf {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Usage: {} <settings.yaml>", args[0]);
        exit(1);
    }

    PathBuf::from(&args[1])
}

/// Load the settings from the given file.
pub fn load_settings(settings_filepath: &Path) -> Settings {
    let file_string = read_to_string(settings_filepath).expect("Unable to read settings file");

    let settings: Settings =
        serde_yaml::from_str(&file_string).expect("Unable to parse settings file");

    if !settings.is_valid() {
        println!("Invalid settings file: {}", settings_filepath.display());
        exit(1);
    }

    settings
}

pub fn save_settings(settings: &Settings, settings_filepath: &Path) {
    let file_string =
        serde_yaml::to_string(&settings).expect("Unable to serialise settings to string");

    write(settings_filepath, file_string).expect("Unable to write settings file");
}

/// Create the output directory if it does not already exist.
pub fn create_output_directory(_settings: &Settings) -> PathBuf {
    let output_directory = PathBuf::from("output");

    if !output_directory.exists() {
        create_dir(&output_directory).expect("Unable to create output directory");
    }

    output_directory
}
