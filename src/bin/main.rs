use image::{save_buffer, ColorType};
use palette::LinSrgba;
use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
    sync::Mutex,
};
use structopt::StructOpt;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[derive(StructOpt)]
struct Cli {
    /// Number of samples per frame.
    #[structopt(short = "n", long = "num")]
    samples_per_frame: usize,

    /// Image resolution.
    #[structopt(short = "r", long = "resolution")]
    resolution: String,

    /// Parameters file path.
    /// The file should be a JSON file
    /// representation of a `Parameters` object.
    #[structopt(short = "p", long = "parameters")]
    parameters_file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    /// Output image directory.
    pub output_image_directory: String,

    /// Output image filename.
    pub output_image_filename: String,
}

trait Canvas {
    /// Get the resolution of the canvas.
    fn resolution(&self) -> (usize, usize);

    /// Get the colour of the pixel at the given coordinates.
    fn pixel_colour(&self, x: usize, y: usize) -> LinSrgba;

    /// Set the colour of the pixel at the given coordinates.
    fn set_pixel_colour(&mut self, x: usize, y: usize, colour: LinSrgba);
}

struct Screen {
    /// Window.
    window: Window,
    /// Event loop.
    event_loop: EventLoop<()>,
    /// Pixels buffer.
    pixels: Pixels,
}

impl Canvas for Pixels {
    fn resolution(&self) -> (usize, usize) {
        (
            self.texture().width() as usize,
            self.texture().height() as usize,
        )
    }

    fn pixel_colour(&self, x: usize, y: usize) -> LinSrgba {
        let index = y * self.resolution().0 + x;
        let pixel: [u8; 4] = self.frame()[index * 4..(index + 1) * 4].try_into().unwrap();
        LinSrgba::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
            pixel[3] as f32 / 255.0,
        )
    }

    fn set_pixel_colour(&mut self, x: usize, y: usize, colour: LinSrgba) {
        let index = y * self.resolution().0 + x;
        let rgba_bytes: [u8; 4] = colour.into_format().into();
        self.frame_mut()[index * 4..(index + 1) * 4].copy_from_slice(&rgba_bytes);
    }
}

impl Screen {
    /// Construct a new instance.
    pub fn new(width: usize, height: usize) -> Screen {
        // Create the window.
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title("Photo")
                .with_transparent(true)
                .with_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        // Create the display buffer.
        let (width, height) = window.inner_size().into();
        let surface_texture = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();

        Screen {
            window,
            event_loop,
            pixels,
        }
    }

    pub fn run<F>(&mut self, mut simulate: F)
    where
        F: FnMut() -> Vec<(usize, [u8; 4])> + 'static,
    {
        self.event_loop.run_return(|event, _, control_flow| {
            Self::handle_event(&event, control_flow, &mut self.pixels);

            let changes = simulate();
            if changes.is_empty() {
                self.window.request_redraw();
                *control_flow = ControlFlow::Exit;
            }

            for (index, colour) in changes {
                self.pixels.frame_mut()[index * 4..(index + 1) * 4].copy_from_slice(&colour);
            }

            self.window.request_redraw();
        });
    }

    fn handle_event(event: &Event<()>, control_flow: &mut ControlFlow, pixels: &mut Pixels) {
        match event {
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => pixels
                .resize_surface(new_size.width, new_size.height)
                .unwrap(),
            _ => (),
        }
    }

    fn save(&self, output_file_path: &PathBuf) {
        let (width, height) = self.pixels.resolution();
        save_buffer(
            output_file_path,
            self.pixels.frame(),
            width as u32,
            height as u32,
            ColorType::Rgba8,
        )
        .unwrap();
        println!("Saved output.png");
    }
}

fn main() {
    // Read the input.
    let (requested_width, requested_height, samples_per_frame, output_file_path) = init();

    // Initialise the screen.
    let mut screen = Screen::new(requested_width, requested_height);

    // Set the draw order.
    let (width, height): (u32, u32) = screen.window.inner_size().into();
    let total_pixels = (width * height) as usize;
    let mut order = (0..total_pixels).collect::<Vec<usize>>();

    use rand::{prelude::SliceRandom, thread_rng};
    order.shuffle(&mut thread_rng());

    // Create an atomically reference counted variable to store the current state.
    use std::sync::Arc;
    let state = Arc::new(Mutex::new(order));

    // Run the simulation.
    let num_threads = num_cpus::get();
    screen.run(move || {
        let changes: Vec<_> = (0..num_threads)
            .into_par_iter()
            .map(|_| {
                let mut state = state.lock().unwrap();
                let final_length = state.len().saturating_sub(samples_per_frame / num_threads);
                let tail = state.split_off(final_length);

                let mut changes = Vec::with_capacity(samples_per_frame / num_threads as usize);
                for n in tail {
                    changes.push((n, process()));
                }

                changes
            })
            .collect();

        changes.into_iter().flatten().collect()
    });

    // Save the image.
    screen.save(&output_file_path);
}

fn init() -> (usize, usize, usize, PathBuf) {
    // Command line arguments.
    let args = Cli::from_args();
    let (width, height) = parse_resolution_string(&args.resolution);

    // Load parameters from file.
    let params_json_str = read_to_string(args.parameters_file_path).unwrap();
    let params: Parameters = serde_json::from_str(&params_json_str).unwrap();

    // Create output directory.
    create_dir_all(&params.output_image_directory).unwrap();

    (
        width,
        height,
        args.samples_per_frame,
        PathBuf::from(params.output_image_directory).join(params.output_image_filename),
    )
}

fn parse_resolution_string(resolution: &str) -> (usize, usize) {
    let mut split = resolution.split('x');
    let width = split.next().unwrap().parse::<usize>().unwrap();
    let height = split.next().unwrap().parse::<usize>().unwrap();
    (width, height)
}

pub fn process() -> [u8; 4] {
    for _ in 0..10000 {
        rand::random::<f32>();
    }
    LinSrgba::new(1.0, 1.0, 1.0, 1.0).into_format().into()
}
