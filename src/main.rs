use photo::{
    geometry::{Mesh, Scene},
    run,
    uniforms::{Camera, Settings},
};

fn main() {
    pollster::block_on(start());
}

async fn start() {
    let resolution = [512, 512];

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

    // let mesh_icosphere = Mesh::load("assets/meshes/icosphere.obj");
    // let mesh_a = Mesh::load("assets/meshes/a.obj");
    // let mesh_b = Mesh::load("assets/meshes/b.obj");
    // let mesh_c = Mesh::load("assets/meshes/c.obj");
    // let mesh_cone = Mesh::load("assets/meshes/cone.obj");
    // let mesh_cubes = Mesh::load("assets/meshes/cubes.obj");
    let mesh_torus = Mesh::load("assets/meshes/torus.obj");
    let mesh_planet = Mesh::load("assets/meshes/planet.obj");
    let mesh_tree = Mesh::load("assets/meshes/tree.obj");
    let scene = Scene::new(vec![mesh_torus, mesh_planet, mesh_tree]);

    let settings = Settings::new(resolution);

    run::with_window(resolution, settings, camera, scene).await;
}
