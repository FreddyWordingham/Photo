use std::{error::Error, path::Path};

use photo::{input::Parameters, VERSION};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Photo! Version: {}", VERSION);

    // Load parameters from file
    let parameters_path = Path::new("input/parameters.yaml");
    let parameters = Parameters::load(parameters_path)?;

    // Validate parameters
    parameters.validate()?;

    // Build world
    let _settings = parameters.build_settings();
    let spectra = parameters.build_spectra()?;
    let materials = parameters.build_materials(&spectra)?;
    let meshes = parameters.build_meshes();
    let _entities = parameters.build_entities(&materials, &meshes)?;
    let _lights = parameters.build_lights();
    let _cameras = parameters.build_cameras();
    drop(parameters);

    // let scene = parameters.create_scene(&settings, &resources);

    Ok(())
}
