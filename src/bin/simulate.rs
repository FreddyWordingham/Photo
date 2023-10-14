fn main() {
    pollster::block_on(run());
}

async fn run() {
    let n_body = init_n_body();

    let nrows = 16 * 64;
    let ncols = 16 * 64;
    let grav_force = 1.0;
    let zoom = nrows.min(ncols) as f32 / 64.0;

    let sim = photo::simulation::NBody::new(n_body, nrows, ncols, grav_force, zoom).await;

    let mut image = photo::Image::new(nrows as usize, ncols as usize, [0.1, 0.1, 0.1, 1.0]);

    for n in 0..10000 {
        for _ in 0..100 {
            sim.run().await;
        }
        sim.render(&mut image).await;
        // image.save(&format!("output/massive_particles.png"));

        // let duration = std::time::Duration::from_millis(100);
        // std::thread::sleep(duration);
        image.save(&format!("output/massive_particles_{0:06}.png", n));
        println!("n = {}", n);
    }
}

fn init_n_body() -> photo::simulation::NBodyInit {
    let mut rng = rand::thread_rng();

    let mut n_body = photo::simulation::NBodyInit::default();
    n_body.add_massive_particle([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1.0e2);
    n_body.add_massive_particle([0.0, -5.0, 0.0], [-4.5, 0.0, 0.0], 0.01);

    n_body.add_ghost_disk(
        &mut rng,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        16.0,
        64 * 64 * 64 * 15,
        10.0,
    );

    n_body
}
