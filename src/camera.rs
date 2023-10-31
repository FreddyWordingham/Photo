pub struct Camera {
    zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self { zoom: 0.15 }
    }

    pub fn is_valid(&self) -> bool {
        self.zoom > 0.0
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self as *const Self as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}
