use bytemuck;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Settings {
    pub x: f32,
    pub y: f32,
}
