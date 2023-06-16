use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Common entry point for all platforms.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start() {
    init_logger();

    let scale = 0.5;
    let resolution = (1920.0 * scale, 1080.0 * scale);

    let (event_loop, window) = init_window(resolution);
    process_loop(event_loop, window);
}

/// Initialize the logger.
/// If the target is wasm32, use console_log.
/// Otherwise, use env_logger.
pub fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
            log::info!("WASM logger initialized.");
        } else {
            env_logger::init();
            log::info!("Standard logger initialized.");
        }
    }
}

/// Initialize the window.
/// If the target is wasm32, use web_sys.
fn init_window(resolution: (f64, f64)) -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Photo!")
        .with_inner_size(PhysicalSize::new(resolution.0, resolution.1))
        .build(&event_loop)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("render")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    (event_loop, window)
}

/// Main process loop.
fn process_loop(event_loop: EventLoop<()>, window: Window) {
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}
