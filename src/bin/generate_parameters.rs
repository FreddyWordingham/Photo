use photo::input::{
    CameraBuilder, GradientBuilder, InstanceBuilder, MaterialBuilder, Parameters, SettingsBuilder,
};
use std::{fs::create_dir_all, path::Path, path::PathBuf};

fn main() {
    // Create output directory if it doesn't exist.
    create_dir_all(Path::new("output")).expect("Unable to create output directory");

    let settings = SettingsBuilder::new("output");
    let gradients = vec![(
        "white".to_string(),
        GradientBuilder::new(vec![0xaaaf, 0xffff]),
    )]
    .into_iter()
    .collect();
    let materials = vec![
        (
            "plastic".to_string(),
            MaterialBuilder::Diffuse {
                gradient_id: "white".to_string(),
            },
        ),
        (
            "mirror".to_string(),
            MaterialBuilder::Reflective {
                gradient_id: "white".to_string(),
                reflectivity: 0.9,
            },
        ),
        (
            "glass".to_string(),
            MaterialBuilder::Refractive {
                gradient_id: "white".to_string(),
                refractive_index: 1.5,
            },
        ),
    ]
    .into_iter()
    .collect();
    let meshes = vec![("cube".to_string(), PathBuf::from("resources/cube.obj"))]
        .into_iter()
        .collect();
    let instances = vec![
        (
            "left_cube".to_string(),
            InstanceBuilder::new("cube".to_string(), "plastic".to_string())
                .with_translation([-1.0, 0.0, 0.0]),
        ),
        (
            "right_cube".to_string(),
            InstanceBuilder::new("cube".to_string(), "plastic".to_string())
                .with_translation([1.0, 0.0, 0.0])
                .with_rotation([10.0, 15.0, 20.0])
                .with_scale(0.5),
        ),
    ]
    .into_iter()
    .collect();
    let cameras = vec![CameraBuilder::new(
        [-4.0, 8.0, 12.0],
        [0.0, 0.0, 0.0],
        90.0,
        Some(2),
        [270, 480],
        [4, 4],
    )];

    let parameters = Parameters::new(settings, gradients, materials, meshes, instances, cameras);
    parameters.save(Path::new("output/parameters.yaml"));
}
