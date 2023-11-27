use photo::{input::Parameters, render::run};
use std::path::Path;

fn main() {
    let parameters = Parameters::load(Path::new("input/parameters.yaml"));

    debug_assert!({
        println!("{}", parameters.as_yaml());
        if parameters.is_valid() {
            println!("Parameters are valid!");
            true
        } else {
            println!("Parameters are invalid!");
            false
        }
    });

    let settings = parameters.settings();
    let resources = parameters.load_resources(&settings);
    let scene = parameters.create_scene(&settings, &resources);
    let cameras = parameters.create_cameras();
    drop(parameters);

    for (n, camera) in cameras.iter().enumerate() {
        let camera_id = format!("camera_{}", n);
        run::render(&settings, &scene, &camera_id, &camera);
    }
}
