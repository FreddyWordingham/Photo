pub struct Settings {
    pub resolution: [u32; 2],
}

impl Settings {
    pub fn new(resolution: [u32; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&x| x > 0));

        Self { resolution }
    }

    pub fn is_valid(&self) -> bool {
        self.resolution.iter().all(|&x| x > 0)
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
