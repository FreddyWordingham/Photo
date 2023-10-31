use crate::{Camera, Controls, Render, Scene, Settings};

const PIXEL_SIZE: u32 = 1; // Screen pixels per logical pixel

pub async fn with_window(resolution: [u32; 2], settings: Settings, camera: Camera, scene: Scene) {
    debug_assert!(resolution.iter().all(|&x| x > 0));
    debug_assert!(settings.is_valid());
    debug_assert!(camera.is_valid());
    debug_assert!(scene.is_valid());

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

    let mut render = Render::new(resolution, settings, camera, scene, window).await;
    let mut controls = Controls::new();

    event_loop
        .run(move |event, control_flow| match event {
            winit::event::Event::WindowEvent {
                window_id,
                ref event,
            } if window_id == render.hardware.window.id() => match event {
                winit::event::WindowEvent::CloseRequested => {
                    println!("Window Closed!");
                    control_flow.exit();
                }
                winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            logical_key:
                                winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape),
                            ..
                        },
                    ..
                } => {
                    println!("Escape Key Pressed!");
                    control_flow.exit();
                }
                winit::event::WindowEvent::KeyboardInput { event, .. } => {
                    controls.keyboard_input(&event, &render);
                }
                winit::event::WindowEvent::Resized(new_size) => {
                    println!("Resized to {:?}", new_size);
                    render.resize(*new_size);
                }
                _ => {}
            },
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { .. } => {
                    // controls.mouse_moved(delta.0, delta.1);
                }
                _ => {}
            },
            winit::event::Event::AboutToWait => {
                render.update();
                controls.update_camera(&mut render.camera);
                render.render(controls.draw_scene_pipeline_index).expect("Photo ERROR!: Failed to render!");

                // Request redraw
                render.hardware.window.request_redraw();
            }
            _ => (),
        })
        .expect("Photo ERROR!: Event loop failed!")
}
