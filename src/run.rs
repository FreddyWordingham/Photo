use crate::{Camera, Render, Scene, Settings};

use winit::event::Event;

const PIXEL_SIZE: u32 = 1; // Screen pixels per logical pixel

pub async fn with_window(resolution: [u32; 2], settings: Settings, camera: Camera, scene: Scene) {
    let [width, height] = resolution;

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("Photo")
        .with_inner_size(winit::dpi::LogicalSize::new(
            width * PIXEL_SIZE,
            height * PIXEL_SIZE,
        ))
        .build(&event_loop)
        .unwrap();

    let render = Render::new(resolution, settings, camera, scene, window).await;

    event_loop
        .run(move |event, _control_flow| match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == render.hardware.window.id() => {
                println!("{:?}", event);
            }
            _ => (),
        })
        .expect("Photo ERROR!: Event loop failed!")
}
