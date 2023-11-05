use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::world::Camera;

/// Runtime camera settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBuilder {
    /// The position of the camera. [x, y, z]
    position: [f64; 3],
    /// The target of the camera. [x, y, z]
    target: [f64; 3],
    /// Horizontal field of view of the camera. [degrees].
    field_of_view: f64,
    /// The resolution of the image in pixels. [rows, columns]
    resolution: [usize; 2],
    /// The resolution of each tile in pixels. [rows, columns]
    tile_resolution: [usize; 2],
}

impl CameraBuilder {
    /// Construct a new instance.
    pub fn new(
        position: [f64; 3],
        target: [f64; 3],
        field_of_view: f64,
        resolution: [usize; 2],
        tile_resolution: [usize; 2],
    ) -> Self {
        debug_assert!(position.iter().all(|p| p.is_finite()));
        debug_assert!(target.iter().all(|t| t.is_finite()));
        debug_assert!(
            position
                .iter()
                .zip(target.iter())
                .fold(0.0, |acc, (p, t)| acc + (p - t).abs())
                > 0.0
        );
        debug_assert!(field_of_view > 0.0);
        debug_assert!(field_of_view < 180.0);
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);
        debug_assert!(tile_resolution[0] > 0);
        debug_assert!(tile_resolution[1] > 0);
        debug_assert!(resolution[0] % tile_resolution[0] == 0);
        debug_assert!(resolution[1] % tile_resolution[1] == 0);

        Self {
            position,
            target,
            field_of_view,
            resolution,
            tile_resolution,
        }
    }

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
