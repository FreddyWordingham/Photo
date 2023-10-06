use bytemuck;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Chunk {
    pub col: [f32; 4],
}

impl Chunk {
    pub fn new(col: [f32; 4]) -> Self {
        Self { col }
    }
}
