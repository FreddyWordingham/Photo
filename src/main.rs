use photo::{run, Camera, Scene, Settings};

fn main() {
    println!("Hello, world!");

    pollster::block_on(start());
}

async fn start() {
    let resolution = [800, 600];
    let camera = Camera {};
    let scene = Scene {};
    let settings = Settings {};

    run::with_window(resolution, settings, camera, scene).await;
}
