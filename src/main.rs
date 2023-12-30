use std::{error::Error, path::Path};

use photo::{input::Parameters, render, world::Scene};

fn main() -> Result<(), Box<dyn Error>> {
    // Load parameters from file
    let parameters_path = Path::new("input/parameters.yaml");
    let parameters = Parameters::load(parameters_path)?;

    // Validate parameters
    parameters.validate()?;

    let mesh_bvh_max_children = 2;
    let mesh_bvh_max_depth = 100;
    let scene_bvh_max_children = 2;
    let scene_bvh_max_depth = 100;

    // Build world components
    let settings = parameters.build_settings();
    let spectra = parameters.build_spectra()?;
    let materials = parameters.build_materials(&spectra)?;
    let meshes = parameters.build_meshes(mesh_bvh_max_children, mesh_bvh_max_depth)?;
    let entities = parameters.build_entities(&materials, &meshes)?;
    let lights = parameters.build_lights();
    let cameras = parameters.build_cameras();
    drop(parameters);

    // Build scene
    let scene = Scene::new(
        lights,
        entities,
        scene_bvh_max_children,
        scene_bvh_max_depth,
    );

    // Render images
    for (name, camera) in cameras.iter() {
        render::run::parallel(&settings, &scene, camera, name)?;
    }

    Ok(())
}
