pub struct Settings {
    pub resolution: [u32; 2],
    padding: [u32; 2],
}

impl Settings {
    pub fn new(resolution: [u32; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&x| x > 0));

        Self {
            resolution,
            padding: [0; 2],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.resolution.iter().all(|&x| x > 0)
    }

    pub fn as_buffer(&self) -> Vec<u32> {
        let mut buffer = self.resolution.to_vec();
        buffer.append(self.padding.to_vec().as_mut());

        buffer
    }
}
