use photo::{input::Parameters, render::run, world::Scene};
use std::path::Path;

fn main() {
    println!("PHOTO!");

    let parameters = Parameters::load(Path::new("input/parameters.yaml"));
    println!("{}", parameters.as_yaml());
    let resources = parameters.load_resources();
    let cameras = parameters.create_cameras();
    let scene = Scene::new(&resources);

    for camera in cameras {
        run::render(&scene, &camera);
    }
}
