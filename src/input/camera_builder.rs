use serde::{Deserialize, Serialize};

/// Camera parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBuilder {
    resolution: [usize; 2],
    num_tiles: [usize; 2],
    super_sampling: Option<usize>,
}

impl CameraBuilder {
    /// Construct a new instance.
    pub fn new(
        resolution: [usize; 2],
        num_tiles: [usize; 2],
        super_sampling: Option<usize>,
    ) -> Self {
        let new = Self {
            resolution,
            num_tiles,
            super_sampling,
        };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the camera parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.num_tiles[0] > 0
            && self.num_tiles[1] > 0
            && self.super_sampling.unwrap_or(1) > 0
    }

    /// Get the resolution.
    pub fn resolution(&self) -> &[usize; 2] {
        &self.resolution
    }

    /// Get the number of tiles.
    pub fn num_tiles(&self) -> &[usize; 2] {
        &self.num_tiles
    }

    /// Get the super sampling.
    pub fn super_sampling(&self) -> Option<usize> {
        self.super_sampling
    }
}
