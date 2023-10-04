use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use photo::canvas::Canvas;
use photo::keypress::handle_keypress;
use photo::state::State;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pixels Example")
        .with_inner_size(LogicalSize::new(400.0 * 3.0, 300.0 * 3.0))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(100, 75, surface_texture)?
    };

    let mut canvas = Canvas::new(100, 75);
    let mut state = State { save_image: false };

    let mut x = 0;
    let y = 75 / 2;
    let r = 15;
    event_loop.run(move |event, _, control_flow| {
        if x > 100 {
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
            _ => (),
        }

        // Handle controls
        {
            if state.save_image {
                canvas.save_to_png("image.png");
                state.save_image = false;
            }
        }

        canvas.clear_background([0x00, 0x77, 0x00, 0xff]);
        canvas.draw_circle(x, y, r, [0xff, 0x00, 0x00, 0xff]);
        canvas.render(pixels.frame_mut());

        pixels.render().expect("Failed to render")
    });
}
