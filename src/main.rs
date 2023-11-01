use photo::{run, Camera, Scene, Settings};

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let resolution = [800, 800];

    let eye_position = [0.0, 5.0, 3.0];
    let look_at = [0.0, 0.0, 0.0];
    let upward_direction = [0.0, 0.0, 1.0];
    let aspect_ratio = resolution[0] as f32 / resolution[1] as f32;
    let fov_x = 40.0;
    let zoom = 1.0;

    let camera = Camera::new(
        eye_position,
        look_at,
        upward_direction,
        aspect_ratio,
        fov_x,
        zoom,
    );

    let mut scene = Scene::new();
    println!("Outer AABB mins: {:?}", scene.aabb().mins());
    println!("Outer AABB maxs: {:?}", scene.aabb().maxs());

    // scene.load_mesh("assets/meshes/triangle.obj");
    // scene.load_mesh("assets/meshes/square.obj");
    // scene.load_mesh("assets/meshes/circle.obj");
    // scene.load_mesh("assets/meshes/cube.obj");
    // scene.load_mesh("assets/meshes/icosphere.obj");
    scene.load_mesh("assets/meshes/torus.obj");
    println!("Outer AABB mins: {:?}", scene.aabb().mins());
    println!("Outer AABB maxs: {:?}", scene.aabb().maxs());

    scene.load_mesh("assets/meshes/cone.obj");
    println!("Outer AABB mins: {:?}", scene.aabb().mins());
    println!("Outer AABB maxs: {:?}", scene.aabb().maxs());

    let settings = Settings::new(resolution);

    run::with_window(resolution, settings, camera, scene).await;
}
