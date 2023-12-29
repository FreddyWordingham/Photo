use std::{error::Error, path::Path};

use nalgebra::{Point3, Unit, Vector3};

use photo::{geometry::Ray, input::Parameters, VERSION};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Photo! Version: {}", VERSION);

    // Load parameters from file
    let parameters_path = Path::new("input/parameters.yaml");
    let parameters = Parameters::load(parameters_path)?;

    // Validate parameters
    parameters.validate()?;

    let bvh_max_children = 4;
    let bvh_max_depth = 100;

    // Build world
    let _settings = parameters.build_settings();
    let spectra = parameters.build_spectra()?;
    let materials = parameters.build_materials(&spectra)?;
    let meshes = parameters.build_meshes(bvh_max_children, bvh_max_depth)?;
    let _entities = parameters.build_entities(&materials, &meshes)?;
    let _lights = parameters.build_lights();
    let _cameras = parameters.build_cameras();
    drop(parameters);

    let min = -1.5;
    let max = 1.5;
    let delta = 0.01;

    let mut x = min;
    while x <= max {
        let ray = Ray::new(
            Point3::new(0.0, x, 10.0),
            Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0)),
        );

        let intersects = meshes["square"].ray_intersect(&ray);

        println!("x: {} - {}", x, intersects);

        x += delta;
    }

    // let scene = parameters.create_scene(&settings, &resources);

    Ok(())
}
