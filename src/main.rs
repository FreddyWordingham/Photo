use photo::{run, Camera, Scene, Settings};

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let resolution = [800, 600];
    let camera = Camera::new();
    let scene = Scene::new();
    let settings = Settings::new();

    run::with_window(resolution, settings, camera, scene).await;
}
