use pixels::{Error, Pixels, SurfaceTexture};
use rayon::prelude::*;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use photo::canvas::Canvas;
use photo::keypress::handle_keypress;
use photo::state::State;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const FRAMES_PER_SECOND: f64 = 10000.0;
const SCALE: u32 = 3;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixels Example")
        .with_inner_size(LogicalSize::new(
            (WIDTH * SCALE) as f32,
            (HEIGHT * SCALE) as f32,
        ))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut state = State { save_image: false };

    let mut last_frame_time = Instant::now();
    let frame_duration = Duration::from_millis((1000.0 / FRAMES_PER_SECOND) as u64);

    let mut x = 0;
    let y = HEIGHT / 2;
    let r = 15;
    event_loop.run(move |event, _, control_flow| {
        if x > WIDTH {
            x = 0;
        }
        x += 1;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        handle_keypress(keycode, input.state, control_flow, &mut state);
                    }
                }
                WindowEvent::Resized(new_size) => pixels
                    .resize_surface(new_size.width, new_size.height)
                    .expect("Failed to resize surface"),
                _ => (),
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let elapsed = now - last_frame_time;
                if elapsed >= frame_duration {
                    last_frame_time = now;
                    // Your update logic here
                } else {
                    std::thread::sleep(frame_duration - elapsed);
                }
            }
            _ => (),
        }

        // Handle controls
        {
            if state.save_image {
                canvas.save_to_png("image.png");
                state.save_image = false;
            }
        }

        random_white_pixels(&mut canvas);
        canvas.render(pixels.frame_mut());
        pixels.render().expect("Failed to render");

        winit::event_loop::ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(10000));
    });
}

use rand::Rng;
use std::sync::{Arc, Mutex};
fn random_white_pixels(canvas: &mut Canvas) {
    let n_threads = 4;
    let n_pixels_per_thread = 1;

    let canvases = Arc::new(Mutex::new(Vec::new()));

    (0..n_threads).into_par_iter().for_each(|_| {
        let mut rng = rand::thread_rng();
        let mut canvas_copy = canvas.clone();
        for _ in 0..n_pixels_per_thread {
            let x = rng.gen_range(0..canvas.width());
            let y = rng.gen_range(0..canvas.height());
            canvas_copy.draw_pixel(x, y, [0xff, 0xff, 0xff, 0xff]); // Make sure draw_pixel is thread-safe
        }

        canvases.lock().unwrap().push(canvas_copy);
    });

    for c in canvases.lock().unwrap().iter() {
        canvas.combine(c);
    }
}
