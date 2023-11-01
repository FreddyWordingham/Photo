pub struct AABB {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
}

impl AABB {
    pub fn new(mins: [f32; 3], maxs: [f32; 3]) -> Self {
        debug_assert!(mins.iter().zip(maxs.iter()).all(|(min, max)| min <= max));

        Self { mins, maxs }
    }
}
