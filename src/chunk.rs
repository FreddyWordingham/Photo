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

impl Chunk {
    pub fn new(col: [f32; 4], x: f32) -> Self {
        Self {
            col,
            x,
            pad_a: 0.0,
            pad_b: 0.0,
            pad_c: 0.0,
        }
    }
}
