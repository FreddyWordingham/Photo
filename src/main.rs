use photo::{
    print_info, run,
    util::{parse_resolution_string, title},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};
use structopt::StructOpt;

/// Input read from the command line.
#[derive(StructOpt)]
pub struct Cli {
    /// Image resolution.
    #[structopt(short = "r", long = "resolution")]
    resolution: String,

    /// Parameters filepath.
    #[structopt(short = "p", long = "parameters")]
    parameters_filepath: PathBuf,
}

/// Input read from the parameters file.
#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    /// Output directory.
    pub output_directory: PathBuf,
}

fn main() {
    title("Photo!");
    let (resolution, _output_dir) = setup();
    run(resolution);
}

/// Read the input from the command line and parameters file,
/// and create the output directory if it doesn't exist.
/// Return the requested resolution and output filepath.
fn setup() -> ((f64, f64), PathBuf) {
    // Command line arguments.
    let args = Cli::from_args();
    let resolution = parse_resolution_string(&args.resolution);

    // Parameters file.
    let parameters_string =
        read_to_string(&args.parameters_filepath).expect("Failed to read parameters file.");
    let parameters: Parameters =
        serde_json::from_str(&parameters_string).expect("Failed to parse parameters file.");

    // Create output directory if it doesn't exist.
    if !parameters.output_directory.exists() {
        create_dir_all(&parameters.output_directory).expect("Failed to create output directory.");
    }

    // Print info.
    print_info!("Width", resolution.0, "px");
    print_info!("Height", resolution.1, "px");
    print_info!("Output directory", parameters.output_directory.display());

    (resolution, parameters.output_directory)
}
