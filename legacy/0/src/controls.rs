use winit::event::{ElementState, VirtualKeyCode};
use winit::event_loop::ControlFlow;

use crate::state::State;

pub fn handle_keypress(
    keycode: VirtualKeyCode,
    element: ElementState,
    control_flow: &mut ControlFlow,
    state: &mut State,
) {
    if keycode == VirtualKeyCode::Escape {
        *control_flow = ControlFlow::Exit;
    } else if keycode == VirtualKeyCode::Space {
        if element == ElementState::Released {
            state.save_image = true;
        }
    }
}
