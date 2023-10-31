use photo::{run, Camera, Scene, Settings};

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let resolution = [800, 800];
    let camera = Camera::new();

    let mut scene = Scene::new();
    scene.load_mesh("assets/meshes/icosphere.obj");
    // scene.load_mesh("assets/meshes/cube.obj");
    // scene.load_mesh("assets/meshes/torus.obj");

    let settings = Settings::new(resolution);

    run::with_window(resolution, settings, camera, scene).await;
}
