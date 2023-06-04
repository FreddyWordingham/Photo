use photo::util::{parse_resolution_string, title};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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
    pub output_directory: String,

    /// Output filename.
    pub output_filename: String,
}

fn main() {
    title("Photo!");
    let (requested_resolution, output_file_path) = read_input();
    println!("Requested resolution  : {:?}", requested_resolution);
    println!("Output file path      : {:?}", output_file_path);
}

fn read_input() -> ((usize, usize), PathBuf) {
    let args = Cli::from_args();
    let requested_resolution = parse_resolution_string(&args.resolution);
    let output_filepath = args.parameters_filepath;
    (requested_resolution, output_filepath)
}
