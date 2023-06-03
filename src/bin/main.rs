use palette::LinSrgba;
use pixels::{Pixels, SurfaceTexture};
use rand::random;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string};
use structopt::StructOpt;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use photo::Image;

#[derive(StructOpt)]
struct Cli {
    /// Number of samples per frame.
    #[structopt(short = "n", long = "num")]
    samples_per_frame: u32,

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

fn main() {
    // Command line arguments.
    let args = Cli::from_args();
    let (width, height) = parse_resolution_string(&args.resolution);

    // Load parameters from file.
    let params_json_str = read_to_string(args.parameters_file_path).unwrap();
    let params: Parameters = serde_json::from_str(&params_json_str).unwrap();

    // Create output directory.
    create_dir_all(&params.output_image_directory).unwrap();

    // Create window.
    let (event_loop, window, mut pixels) = create_window(width, height);

    let mut img = Image::new(width, height);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                update_display_buffer(&mut pixels, &img);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => pixels
                .resize_surface(new_size.width, new_size.height)
                .unwrap(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            _ => (),
        }

        for _ in 0..args.samples_per_frame {
            process(&mut img);
        }

        window.request_redraw();
    });
}

fn parse_resolution_string(resolution: &str) -> (usize, usize) {
    let mut split = resolution.split('x');
    let width = split.next().unwrap().parse::<usize>().unwrap();
    let height = split.next().unwrap().parse::<usize>().unwrap();
    (width, height)
}

fn create_window(width: usize, height: usize) -> (EventLoop<()>, Window, Pixels) {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("Photo")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let pixels = Pixels::new(
        window_size.width as u32,
        window_size.height as u32,
        surface_texture,
    )
    .unwrap();

    return (event_loop, window, pixels);
}

fn update_display_buffer(pixels: &mut Pixels, img: &Image) {
    let (width, _height) = img.resolution();
    pixels
        .frame_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, pixel)| calculate_pixel_colour(width, i, pixel, img));
}

fn calculate_pixel_colour(width: usize, index: usize, pixel: &mut [u8], img: &Image) {
    let x = index % width;
    let y = index / width;
    let col = img.data[[x, y]];
    let slice: [u8; 4] = col.into_format().into();
    pixel.copy_from_slice(&slice);
}

/// Process the image.
pub fn process(img: &mut Image) {
    let (width, height) = img.resolution();
    let x = random::<usize>() % width;
    let y = random::<usize>() % height;
    img.data[[x, y]] = LinSrgba::new(1.0, 1.0, 1.0, 1.0);
}
