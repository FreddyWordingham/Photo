use photo::{
    geometry::{Mesh, Scene, AABB},
    run, Camera, Settings,
};

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

    let mesh_a = Mesh::load("assets/meshes/a.obj");
    let scene = Scene::new(vec![mesh_a]);

    let settings = Settings::new(resolution);

    run::with_window(resolution, settings, camera, scene).await;
}
