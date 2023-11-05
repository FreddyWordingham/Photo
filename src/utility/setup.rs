use crate::input::Settings;

/// Read the command line arguments.
/// Specifically, read the path to the settings file.
pub fn read_command_line_arguments() -> std::path::PathBuf {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <settings.yaml>", args[0]);
        std::process::exit(1);
    }

    std::path::PathBuf::from(&args[1])
}

/// Load the settings from the given file.
pub fn load_settings(settings_filepath: &std::path::Path) -> Settings {
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

pub fn save_settings(settings: &Settings, settings_filepath: &std::path::Path) {
    let file_string =
        serde_yaml::to_string(&settings).expect("Unable to serialise settings to string");

    std::fs::write(settings_filepath, file_string).expect("Unable to write settings file");
}

/// Create the output directory if it does not already exist.
pub fn create_output_directory(_settings: &Settings) -> std::path::PathBuf {
    let output_directory = std::path::PathBuf::from("output");

    if !output_directory.exists() {
        std::fs::create_dir(&output_directory).expect("Unable to create output directory");
    }

    output_directory
}
