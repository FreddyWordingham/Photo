use photo::input::{CameraBuilder, InstanceBuilder, Parameters};
use std::{fs::create_dir_all, path::Path};

fn main() {
    // Create output directory if it doesn't exist.
    create_dir_all(Path::new("output")).expect("Unable to create output directory");

    let meshes = vec![("cube".to_string(), "resources/cube.obj".to_string())]
        .into_iter()
        .collect();
    let instances = vec![
        (
            "left_cube".to_string(),
            InstanceBuilder::new("cube".to_string()).with_translation([-1.0, 0.0, 0.0]),
        ),
        (
            "right_cube".to_string(),
            InstanceBuilder::new("cube".to_string())
                .with_translation([1.0, 0.0, 0.0])
                .with_rotation([10.0, 15.0, 20.0])
                .with_scale(0.5),
        ),
    ]
    .into_iter()
    .collect();
    let cameras = vec![CameraBuilder::new([1920, 1080])];

    let parameters = Parameters::new(meshes, instances, cameras);
    parameters.save(Path::new("output/parameters.yaml"));
}
