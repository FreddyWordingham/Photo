use palette::LinSrgba;
use photo::{
    print_info,
    util::{parse_resolution_string, title},
    Gui,
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
    let (requested_res, output_dir) = setup();
    let mut gui = Gui::new(requested_res);

    gui.run(move || {
        let n = 1000;
        let mut changes = Vec::with_capacity(n);
        for _ in 0..n {
            let x = rand::random::<f64>() * requested_res.0;
            let y = rand::random::<f64>() * requested_res.1;
            let index = x as usize + y as usize * requested_res.0 as usize;
            let colour = LinSrgba::new(1.0, 1.0, 1.0, 1.0).into_format().into();
            changes.push((index, colour));
        }
        changes
    });
}

/// Read the input from the command line and parameters file,
/// and create the output directory if it doesn't exist.
/// Return the requested resolution and output filepath.
fn setup() -> ((f64, f64), PathBuf) {
    // Command line arguments.
    let args = Cli::from_args();
    let requested_res = parse_resolution_string(&args.resolution);

    // Parameters file.
    let params_str =
        read_to_string(&args.parameters_filepath).expect("Failed to read parameters file.");
    let params: Parameters =
        serde_json::from_str(&params_str).expect("Failed to parse parameters file.");

    // Create output directory if it doesn't exist.
    if !params.output_directory.exists() {
        create_dir_all(&params.output_directory).expect("Failed to create output directory.");
    }

    // Print info.
    print_info!("Width", requested_res.0, "px");
    print_info!("Height", requested_res.1, "px");
    print_info!("Output directory", params.output_directory.display());

    (requested_res, params.output_directory)
}
