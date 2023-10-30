pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_valid(&self) -> bool {
        true
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
