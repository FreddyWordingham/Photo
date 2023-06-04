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
}
