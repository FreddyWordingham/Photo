use photo;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    run(64 * 4, 64 * 8, 4);
}

fn run(nrows: u32, ncols: u32, scale: u32) {
    let window = photo::Window::new(nrows, ncols, scale);
    let controller = Cont::new(nrows, ncols);

    window.run(controller);
}

struct Cont {
    image: photo::Image,
    rng: rand::rngs::ThreadRng,
    shader: photo::Shader,
    frame_counter: usize,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Cont {
    pub fn new(nrows: u32, ncols: u32) -> Self {
        debug_assert!(nrows > 0);
        debug_assert!(ncols > 0);

        let hardware = pollster::block_on(photo::Hardware::new());
        let shader = pollster::block_on(photo::Shader::new(
            include_str!("../shaders/blur.wgsl"),
            nrows,
            ncols,
            hardware,
        ));

        Self {
            image: photo::Image::new(nrows as usize, ncols as usize, [0.2, 0.2, 0.2, 1.0]),
            rng: rand::thread_rng(),
            shader,
            frame_counter: 0,
            x: 0.5,
            y: 0.5,
            dx: 0.016634,
            dy: 0.01234,
        }
    }
}

impl photo::Controller for Cont {
    fn handle_keypress(
        &mut self,
        keycode: winit::event::VirtualKeyCode,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        match keycode {
            winit::event::VirtualKeyCode::Escape => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            _ => {
                println!("Other key pressed: {:?}", keycode);
            }
        }
    }

    fn tick(&mut self) {
        println!("FPS: {}", self.frame_counter);
        self.frame_counter = 0;
    }

    fn update(&mut self) {
        let x = self.rng.gen_range(0..self.image.ncols());
        let y = self.rng.gen_range(0..self.image.nrows());

        let radius = self.rng.gen_range(1..10);

        self.image.set_circle(
            y,
            x,
            radius,
            [
                self.rng.gen_range(0.0..1.0),
                self.rng.gen_range(0.0..1.0),
                self.rng.gen_range(0.0..1.0),
                1.0,
            ],
        );

        self.x += self.dx;
        if self.x < 0.0 || self.x >= 1.0 {
            self.x -= self.dx;
            self.dx *= -1.0;
        }
        self.y += self.dy;
        if self.y < 0.0 || self.y >= 1.0 {
            self.y -= self.dy;
            self.dy *= -1.0;
        }

        self.image.set_circle(
            (self.image.nrows() as f32 * self.x) as usize,
            (self.image.ncols() as f32 * self.y) as usize,
            4,
            [1.0, 1.0, 1.0, 1.0],
        );
    }

    fn render(&mut self, pixels: &mut [u8]) {
        self.frame_counter += 1;

        let uniform = [0.0f32, 0.0f32];
        self.shader.write_uniform_to_gpu(&uniform);
        self.shader.write_image_to_gpu(&self.image);

        let mut output_image = self.image.clone();

        pollster::block_on(self.shader.run_shader(&mut output_image));

        pixels.copy_from_slice(output_image.as_u8().as_slice());
    }
}
