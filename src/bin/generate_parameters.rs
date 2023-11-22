use photo::input::{CameraBuilder, Parameters};
use std::{fs::create_dir_all, path::Path};

fn main() {
    // Create output directory if it doesn't exist.
    create_dir_all(Path::new("output")).expect("Unable to create output directory");

    let cameras = vec![CameraBuilder::new([1920, 1080])];

    let parameters = Parameters::new(cameras);
    parameters.save(Path::new("output/parameters.yaml"));
}
