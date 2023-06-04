use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub struct Gui {
    /// Window.
    window: Window,
    /// Event loop.
    event_loop: EventLoop<()>,
    /// Pixels buffer.
    pixels: Pixels,
}

impl Gui {
    /// Construct a new instance.
    pub fn new(requested_resolution: (f64, f64)) -> Self {
        let (width, height) = requested_resolution;

        // Create the window.
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(width, height);
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

        Self {
            window,
            event_loop,
            pixels,
        }
    }

    pub fn run<F>(&mut self, mut compute_changes: F)
    where
        F: FnMut() -> Vec<(usize, [u8; 4])> + 'static,
    {
        self.event_loop.run_return(|event, _, control_flow| {
            Self::handle_event(&event, control_flow, &mut self.pixels);

            let changes = compute_changes();
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
}
