use std::{env::args, fs::create_dir, path::PathBuf, process::exit};

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

/// Create the output directory if it does not already exist.
pub fn create_output_directory(_settings: &Settings) -> PathBuf {
    let output_directory = PathBuf::from("output");

    if !output_directory.exists() {
        create_dir(&output_directory).expect("Unable to create output directory");
    }

    output_directory
}
