pub struct Controls {}

impl Controls {
    pub fn new() -> Self {
        Self {}
    }

    pub fn keyboard_input(&mut self, event: &winit::event::KeyEvent) {
        match event {
            winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                ..
            } => {
                self.process_key_down(code);
            }
            _ => {}
        }
    }

    fn process_key_down(&mut self, code: &winit::keyboard::KeyCode) {
        match code {
            winit::keyboard::KeyCode::KeyQ => {
                println!("PRESSED Q!");
            }
            _ => {
                println!("Unknown Key: {:?}", code);
            }
        }
    }

    pub fn mouse_moved(&mut self, delta_x: f64, delta_y: f64) {
        println!("Mouse Moved: {}, {}", delta_x, delta_y);
    }
}
