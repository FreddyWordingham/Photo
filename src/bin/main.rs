use photo::{input::Parameters, render::run};
use std::{fs::create_dir_all, path::Path};

fn main() {
    println!("PHOTO!");

    let parameters = Parameters::load(Path::new("input/parameters.yaml"));
    println!("{}", parameters.as_yaml());

    if parameters.is_valid() {
        println!("Parameters are valid!");
    } else {
        println!("Parameters are invalid!");
    }

    let settings = parameters.settings();
    let resources = parameters.load_resources();
    let scene = parameters.create_scene(&resources);
    let cameras = parameters.create_cameras();

    drop(parameters);

    let output_directory = settings.output_directory();
    create_dir_all(output_directory).expect("Unable to create output directory");
    for (n, camera) in cameras.iter().enumerate() {
        let camera_output_directory = settings.output_directory().join(&format!("camera_{}", n));
        create_dir_all(camera_output_directory.clone())
            .expect("Unable to create camera output directory");

        run::render(&camera_output_directory, &scene, &camera);
    }
}
