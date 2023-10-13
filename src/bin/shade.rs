use photo;

fn main() {
    let mut image = photo::Image::load("input/test0.png");
    println!("{} rows - {} cols", image.nrows(), image.ncols());

    let hardware = pollster::block_on(photo::Hardware::new());
    let mut shader = pollster::block_on(photo::Shader::new(
        include_str!("../shaders/blur.wgsl"),
        image.nrows() as u32,
        image.ncols() as u32,
        hardware,
    ));
    let uniform = [0.0f32, 0.0f32];
    shader.write_uniform_to_gpu(&uniform);
    shader.write_image_to_gpu(&image);
    pollster::block_on(shader.run_shader(&mut image));

    // image.print([1.0, 1.0, 1.0]);
    image.save("output/test.png");
}
