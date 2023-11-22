use serde::{Deserialize, Serialize};

/// Camera parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBuilder {
    resolution: [usize; 2],
}

impl CameraBuilder {
    /// Construct a new instance.
    pub fn new(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        Self { resolution }
    }

    /// Get the resolution.
    pub fn resolution(&self) -> &[usize; 2] {
        &self.resolution
    }
}
