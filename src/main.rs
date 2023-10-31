use photo::{run, Camera, Scene, Settings};

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let resolution = [800, 600];
    let camera = Camera::new();

    let mut scene = Scene::new();
    scene.load_mesh("assets/meshes/icosphere.obj");

    let settings = Settings::new(resolution);

    run::with_window(resolution, settings, camera, scene).await;
}
