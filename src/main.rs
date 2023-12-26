use std::{error::Error, path::Path};

use photo::{input::Parameters, VERSION};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Photo! Version: {}", VERSION);

    // Load parameters from file
    let parameters_path = Path::new("input/parameters.yaml");
    let parameters = Parameters::load(parameters_path)?;

    // Validate parameters
    parameters.validate()?;

    Ok(())
}
