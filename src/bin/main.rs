use photo::{input::Parameters, render::run};
use std::path::Path;

fn main() {
    println!("PHOTO!");

    let parameters = Parameters::load(Path::new("input/parameters.yaml"));
    println!("{}", parameters.as_yaml());

    if parameters.is_valid() {
        println!("Parameters are valid!");
    } else {
        println!("Parameters are invalid!");
    }

    let resources = parameters.load_resources();
    let scene = parameters.create_scene(&resources);
    let cameras = parameters.create_cameras();

    for camera in cameras {
        run::render(&scene, &camera);
    }
}
