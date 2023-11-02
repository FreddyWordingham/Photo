use crate::{uniforms::Camera, Render};

pub struct Controls {
    pub azimuthal_rotation_rate: f32,
    pub polar_rotation_rate: f32,
    pub magnification_rate: f32,

    pub draw_scene_pipeline_index: usize,
}

impl Controls {
    pub fn new() -> Self {
        Self {
            azimuthal_rotation_rate: 0.0,
            polar_rotation_rate: 0.0,
            magnification_rate: 1.0,
            draw_scene_pipeline_index: 0,
        }
    }

    pub fn mouse_moved(&mut self, delta_x: f64, delta_y: f64) {
        println!("Mouse Moved: {}, {}", delta_x, delta_y);
    }

    pub fn keyboard_input(&mut self, event: &winit::event::KeyEvent, render: &Render) {
        match event {
            winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                state: winit::event::ElementState::Released,
                ..
            } => {
                self.process_key_up(code, render);
            }
            _ => {}
        }
    }

    fn process_key_up(&mut self, code: &winit::keyboard::KeyCode, render: &Render) {
        match code {
            winit::keyboard::KeyCode::KeyW => {
                self.polar_rotation_rate += 0.01;
            }
            winit::keyboard::KeyCode::KeyS => {
                self.polar_rotation_rate -= 0.01;
            }
            winit::keyboard::KeyCode::KeyA => {
                self.azimuthal_rotation_rate -= 0.01;
            }
            winit::keyboard::KeyCode::KeyD => {
                self.azimuthal_rotation_rate += 0.01;
            }
            winit::keyboard::KeyCode::KeyE => {
                self.magnification_rate += 0.01;
            }
            winit::keyboard::KeyCode::KeyQ => {
                self.magnification_rate -= 0.01;
            }
            winit::keyboard::KeyCode::KeyX => {
                self.draw_scene_pipeline_index = (self.draw_scene_pipeline_index + 1)
                    % render.pipelines.draw_scene_pipelines.len();
            }
            _ => {
                println!("Unknown Key: {:?}", code);
            }
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        camera.rotate_azimuthal(self.azimuthal_rotation_rate);
        camera.rotate_polar(self.polar_rotation_rate);
        camera.magnify(self.magnification_rate);
    }
}
