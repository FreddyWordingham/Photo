/// Runtime rendering settings.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    /// The resolution of the image in pixels.
    resolution: [usize; 2],
}

impl Settings {
    /// Construct a new Settings object.
    pub const fn new(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        Self { resolution }
    }

    /// Check that all current settings are valid.
    pub fn are_valid(&self) -> bool {
        self.resolution[0] > 0 && self.resolution[1] > 0
    }
}
