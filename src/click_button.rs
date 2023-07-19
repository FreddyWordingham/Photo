use winit::event::*;

pub struct ClickButton {
    is_just_pressed: bool,
    is_locked: bool,
}

impl ClickButton {
    pub fn new() -> Self {
        Self {
            is_just_pressed: false,
            is_locked: false,
        }
    }

    pub fn process_state(&mut self, state: &ElementState) {
        if *state == ElementState::Pressed && !self.is_just_pressed && !self.is_locked {
            self.is_just_pressed = true;
            self.is_locked = true;
        } else if *state == ElementState::Released {
            self.is_locked = false;
        }
    }

    pub fn triggered(&mut self) -> bool {
        if self.is_just_pressed {
            self.is_just_pressed = false;
            return true;
        }
        false
    }
}
