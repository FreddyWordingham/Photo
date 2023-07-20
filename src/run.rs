use crate::state::State;

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
pub async fn start() {
    init_logger();

    let scale = 0.5;
    let resolution = (1920.0 * scale, 1080.0 * scale);

    let (event_loop, window) = init_window(resolution);
    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CursorMoved { position, .. } => {
                        let col = handle_mouse_move(position, state.size);
                        state.set_clear_colour(col);
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        handle_keypress(input, control_flow)
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            state.window().request_redraw();
        }
        _ => {}
    });
}

/// Initialize the logger.
/// If the target is wasm32, use console_log.
/// Otherwise, use env_logger.
pub fn init_logger() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
            log::info!("WASM logger initialized");
        } else {
            env_logger::init();
            log::info!("Standard logger initialized");
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
            .expect("Couldn't append canvas to document body");
    }

    (event_loop, window)
}

/// Handle a keypress event.
fn handle_keypress(event: &KeyboardInput, control_flow: &mut ControlFlow) {
    match event {
        KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(code),
            ..
        } => match code {
            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
            _ => {
                log::info!("Unhandled keypress: {:?}", code);
            }
        },
        _ => {}
    }
}

/// Handle a mouse movement event.
fn handle_mouse_move(
    position: &winit::dpi::PhysicalPosition<f64>,
    size: winit::dpi::PhysicalSize<u32>,
) -> wgpu::Color {
    wgpu::Color {
        r: position.x / size.width as f64,
        g: position.y / size.height as f64,
        b: 0.3,
        a: 1.0,
    }
}
