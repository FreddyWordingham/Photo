fn main() {
    pollster::block_on(run());
}

async fn run() {
    let n_body = init_n_body();

    let nrows = 128;
    let ncols = 8 * 20;
    let grav_force = 1.0;
    let zoom = nrows.min(ncols) as f32 / 2.0;

    let sim = photo::simulation::NBody::new(n_body, nrows, ncols, grav_force, zoom).await;

    let mut image = photo::Image::new(nrows as usize, ncols as usize, [0.1, 0.1, 0.1, 1.0]);
    sim.run(&mut image).await;
    image.save("output/massive_particles.png");
}

fn init_n_body() -> photo::simulation::NBodyInit {
    let mut rng = rand::thread_rng();

    let mut n_body = photo::simulation::NBodyInit::default();
    n_body.add_massive_particle([0.25, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0e3);
    n_body.add_massive_particle([-0.25, 0.0, 0.0], [0.0, -1.0, 0.0], 1.0e3);
    n_body.add_ghost_field(&mut rng, [0.0, 0.0, 0.0], 1.0, 1000);

    n_body
}
