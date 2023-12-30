use std::{error::Error, path::Path};

use photo::{input::Parameters, render, world::Scene};

fn main() -> Result<(), Box<dyn Error>> {
    // Read command line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: photo <path/to/parameters.yaml>");
        std::process::exit(1);
    }
    let parameters_path = Path::new(&args[1]);

    // Check if parameters file exists.
    if !parameters_path.exists() {
        eprintln!(
            "Error: parameters file `{}` does not exist.",
            parameters_path.display()
        );
        std::process::exit(1);
    }

    // Load parameters from file.
    let parameters = Parameters::load(parameters_path)?;

    // Validate parameters.
    parameters.validate()?;

    // Build world components.
    let settings = parameters.build_settings();
    let spectra = parameters.build_spectra()?;
    let materials = parameters.build_materials(&spectra)?;
    let meshes =
        parameters.build_meshes(settings.mesh_bvh_max_children, settings.mesh_bvh_max_depth)?;
    let entities = parameters.build_entities(&materials, &meshes)?;
    let lights = parameters.build_lights();
    let cameras = parameters.build_cameras();
    drop(parameters);

    // Build scene.
    let scene = Scene::new(
        lights,
        entities,
        settings.scene_bvh_max_children,
        settings.scene_bvh_max_depth,
    );

    // Render images.
    for (name, camera) in cameras.iter() {
        render::run::parallel(&settings, &scene, camera, name)?;
    }

    Ok(())
}
