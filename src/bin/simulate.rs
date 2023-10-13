fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let num_bodies = 1000;
    let mut simulation = photo::simulation::NBody::new(&mut rng, num_bodies);

    let background_colour = [0.1, 0.1, 0.1, 1.0];
    let camera = photo::simulation::Camera::new(background_colour.clone());
    let mut canvas = photo::Image::new(256, 512, background_colour);

    camera.render(&mut canvas, &simulation);
    canvas.save("output/n_body.png");
    for _ in 0..3 {
        for _ in 0..10 {
            simulation.step(0.001);
        }
        camera.render(&mut canvas, &simulation);
        canvas.save("output/n_body.png");
    }
}
