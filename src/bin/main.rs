use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const FRAMES_PER_SECOND: usize = 60;

fn main() {
    run();
}

fn run() {
    let width = WIDTH;
    let height = HEIGHT;
    let frame_time = Duration::from_micros((1000.0 / (FRAMES_PER_SECOND as f64)) as u64);

    // Create a buffer for the window.
    let mut buffer: Vec<u32> = vec![0; width * height];

    // Create the window.
    let mut window =
        Window::new("Photo", width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit framerate
    window.limit_update_rate(Some(frame_time));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0;
        }

        // We unwrap here as we want this code to exit if it fails.
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
