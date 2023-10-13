use crate::Controller;

pub struct Window {
    nrows: u32,
    ncols: u32,
    scale: u32,
}

impl Window {
    pub fn new(nrows: u32, ncols: u32, scale: u32) -> Self {
        debug_assert!(nrows > 0);
        debug_assert!(ncols > 0);
        debug_assert!(scale > 0 && scale <= 4);

        Self {
            nrows,
            ncols,
            scale,
        }
    }

    pub fn run<C: Controller + 'static>(&self, mut controller: C) {
        // Create the window.
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title("Photo Rendering")
            .with_inner_size(winit::dpi::LogicalSize::new(
                (self.ncols * self.scale) as f32,
                (self.nrows * self.scale) as f32,
            ))
            .build(&event_loop)
            .unwrap();

        // Window texture.
        let mut window_texture = {
            let window_size = window.inner_size();
            let surface_texture =
                pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
            pixels::Pixels::new(self.ncols, self.nrows, surface_texture)
                .expect("Failed to create pixels window texture")
        };

        let mut next_tick = std::time::Instant::now();
        event_loop.run(move |event, _, control_flow| {
            // Tick the controller once per second.
            let now = std::time::Instant::now();
            if now >= next_tick {
                controller.tick();
                next_tick = now + std::time::Duration::from_secs(1);
            }

            // Update the controller.
            controller.update();

            // Process events.
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            controller.handle_keypress(keycode, control_flow);
                        }
                    }
                    _ => (),
                },
                winit::event::Event::RedrawRequested(_) => {
                    controller.render(window_texture.frame_mut());
                    window_texture
                        .render()
                        .expect("Failed to render window texture");
                }
                winit::event::Event::MainEventsCleared => window.request_redraw(),
                _ => (),
            }
        });
    }
}
