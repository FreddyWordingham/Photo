use crate::click_button::ClickButton;
use winit::event::*;

pub struct CycleController {
    next_key: VirtualKeyCode,
    back_key: VirtualKeyCode,
    next: ClickButton,
    back: ClickButton,
}

impl CycleController {
    pub fn new(next_key: VirtualKeyCode, back_key: VirtualKeyCode) -> Self {
        Self {
            next_key: next_key,
            back_key: back_key,
            next: ClickButton::new(),
            back: ClickButton::new(),
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
            ..
        } = event
        {
            if *keycode == self.next_key {
                self.next.process_state(state);
                true
            } else if *keycode == self.back_key {
                self.back.process_state(state);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn update_index(&mut self, index: &mut usize, max: usize) {
        if self.next.triggered() {
            *index = (*index + 1) % max;
        } else if self.back.triggered() {
            *index = (*index + max - 1) % max;
        }
    }
}
