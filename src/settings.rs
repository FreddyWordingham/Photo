use bytemuck;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Settings {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    pub v: f32,
}
