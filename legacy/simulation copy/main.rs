use core::num;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("Hello, world!");

    let num_bodies = 100000;
    let grav_force = 0.0001;
    let speed = 0.37;

    let mut rng = rand::thread_rng();
    let mut simulation = photo::simulation::NBody::new(&mut rng, num_bodies, grav_force, speed);

    let background_colour = [0.0, 0.0, 0.0, 1.0];
    let camera_32 = photo::simulation::Camera::new(background_colour.clone(), [0.0, 0.0], 32.0);
    let camera_16 = photo::simulation::Camera::new(background_colour.clone(), [0.0, 0.0], 16.0);
    let camera_8 = photo::simulation::Camera::new(background_colour.clone(), [0.0, 0.0], 8.0);
    let mut canvas = photo::Image::new(128 * 4, 128 * 4, background_colour);

    let mut simulation = simulation.await;

    camera_32.render(&mut canvas, &simulation);
    canvas.save("output/32/n_body.png");
    camera_16.render(&mut canvas, &simulation);
    canvas.save("output/16/n_body.png");
    camera_8.render(&mut canvas, &simulation);
    canvas.save("output/8/n_body.png");

    let total_time = 1.0e2;
    let num_steps = 1000;
    let num_internal_steps = 100;
    let dt = total_time / (num_steps * num_internal_steps) as f32;
    for n in 0..num_steps {
        println!("Step {}/{}", n + 1, num_steps);
        for _ in 0..num_internal_steps {
            pollster::block_on(simulation.step(dt, grav_force));
        }

        camera_32.render(&mut canvas, &simulation);
        canvas.save(&format!("output/32/n_body_{0:03}.png", n));
        camera_16.render(&mut canvas, &simulation);
        canvas.save(&format!("output/16/n_body_{0:03}.png", n));
        camera_8.render(&mut canvas, &simulation);
        canvas.save(&format!("output/8/n_body_{0:03}.png", n));

        // let duration = std::time::Duration::from_millis(100);
        // std::thread::sleep(duration);
    }
}
