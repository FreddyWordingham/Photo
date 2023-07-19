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

    pub fn update_bind_group_index(
        &mut self,
        bind_group_index: &mut usize,
        bind_groups: &[wgpu::BindGroup],
    ) {
        if self.next.triggered() {
            *bind_group_index = (*bind_group_index + 1) % bind_groups.len();
        } else if self.back.triggered() {
            *bind_group_index = (*bind_group_index + bind_groups.len() - 1) % bind_groups.len();
        }
    }
}
