use photo::{input::Parameters, render::run};
use std::path::Path;

fn main() {
    println!("PHOTO!");

    let parameters = Parameters::load(Path::new("input/parameters.yaml"));
    println!("{}", parameters.as_yaml());

    let resources = parameters.load_resources();
    let scene = parameters.create_scene(&resources);
    let cameras = parameters.create_cameras();

    for camera in cameras {
        run::render(&scene, &camera);
    }
}
