use bytemuck;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Chunk {
    pub col: [f32; 4],
    pub x: f32,
    pub pad_a: f32,
    pub pad_b: f32,
    pub pad_c: f32,
}
