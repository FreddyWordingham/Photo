use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::world::Camera;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraSettings {
    /// The position of the camera. [x, y, z]
    pub position: [f64; 3],
    /// The target of the camera. [x, y, z]
    pub target: [f64; 3],
    /// Horizontal field of view of the camera. [degrees].
    pub field_of_view: f64,
    /// The resolution of the image in pixels. [rows, columns]
    pub resolution: [usize; 2],
    /// The resolution of each tile in pixels. [rows, columns]
    pub tile_resolution: [usize; 2],
}

impl CameraSettings {
    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        self.position.iter().all(|p| p.is_finite())
            && self.target.iter().all(|t| t.is_finite())
            && self
                .position
                .iter()
                .zip(self.target.iter())
                .fold(0.0, |acc, (p, t)| acc + (p - t).abs())
                > 0.0
            && self.field_of_view > 0.0
            && self.field_of_view < 180.0
            && self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.tile_resolution[0] > 0
            && self.tile_resolution[1] > 0
            && self.resolution[0] % self.tile_resolution[0] == 0
            && self.resolution[1] % self.tile_resolution[1] == 0
    }

    /// Build a camera from the current settings.
    pub fn build(&self) -> Camera {
        debug_assert!(self.is_valid());

        Camera::new(
            Vector3::from_row_slice(&self.position),
            Vector3::from_row_slice(&self.target),
            self.field_of_view * PI / 180.0,
            self.resolution,
            self.tile_resolution,
        )
    }
}
