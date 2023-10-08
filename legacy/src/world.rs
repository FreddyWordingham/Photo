use bytemuck;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct World {
    pub player_x: f32,
    pub player_y: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            player_x: 0.5,
            player_y: 0.5,
        }
    }
}
