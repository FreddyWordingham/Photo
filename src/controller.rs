use winit;

pub trait Controller {
    fn handle_keypress(
        &mut self,
        keycode: winit::event::VirtualKeyCode,
        control_flow: &mut winit::event_loop::ControlFlow,
    );

    fn tick(&mut self);

    fn update(&mut self);

    fn render(&mut self, pixels: &mut [u8]);
}
