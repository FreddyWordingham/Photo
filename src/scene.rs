const MODEL_VERTICES: &[([f32; 3], f32)] = &[
    ([-0.25, 0.0, 0.25], 0.0),
    ([-0.25, 0.0, -0.25], 0.0),
    ([0.25, 0.0, -0.25], 0.0),
    ([0.25, 0.0, 0.25], 0.0),
];

const MODEL_INDICES: &[([u32; 3], u32)] = &[
    ([0, 1, 3], 0),
    ([1, 2, 3], 0),
    ([1, 2, 3], 0),
    ([0, 1, 3], 0),
    ([0, 1, 3], 0),
    ([1, 2, 3], 0),
];

pub struct Scene {}

impl Scene {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn triangles_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                MODEL_VERTICES.as_ptr() as *const u8,
                MODEL_VERTICES.len() * std::mem::size_of::<([f32; 3], f32)>(),
            )
        }
    }

    pub fn indices_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                MODEL_INDICES.as_ptr() as *const u8,
                MODEL_INDICES.len() * std::mem::size_of::<([u32; 3], u32)>(),
            )
        }
    }
}
