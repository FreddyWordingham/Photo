use nalgebra::Similarity3;

use photo::run;
use photo::world::Instance;
use photo::world::Resources;
use photo::world::Scene;
use photo::Parameters;

fn main() {
    println!("PHOTO!");

    let parameters = Parameters::new();
    let cameras = parameters.create_cameras();
    let resources = parameters.create_resources();
    let instances = init_instances(&resources);
    let scene = Scene::new(&resources, instances);

    for camera in cameras {
        run::render(&scene, &camera);
    }
}

fn init_instances(resources: &Resources) -> Vec<Instance> {
    let instances = vec![
        Instance::new(&resources.meshes()[0], Similarity3::identity()),
        Instance::new(&resources.meshes()[1], Similarity3::identity()),
        Instance::new(&resources.meshes()[2], Similarity3::identity()),
    ];
    instances
}
