use crate::click_button::ClickButton;
use winit::event::*;

pub struct BindGroupController {
    next: ClickButton,
    back: ClickButton,
}

impl BindGroupController {
    pub fn new() -> Self {
        Self {
            next: ClickButton::new(),
            back: ClickButton::new(),
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => match keycode {
                VirtualKeyCode::E => {
                    self.next.process_state(state);
                    true
                }
                VirtualKeyCode::Q => {
                    self.back.process_state(state);
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn update_bindgroup_index(
        &mut self,
        bindgroup_index: &mut usize,
        bindgroups: &[wgpu::BindGroup],
    ) {
        if self.next.triggered() {
            *bindgroup_index = (*bindgroup_index + 1) % bindgroups.len();
        } else if self.back.triggered() {
            *bindgroup_index = (*bindgroup_index + bindgroups.len() - 1) % bindgroups.len();
        }
    }
}
