use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::world::Camera;

/// Camera parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraBuilder {
    position: [f64; 3],
    target: [f64; 3],
    field_of_view: f64,
    super_samples_per_axis: Option<usize>,
    resolution: [usize; 2],
    num_tiles: [usize; 2],
}

impl CameraBuilder {
    /// Construct a new instance.
    pub fn new(
        position: [f64; 3],
        target: [f64; 3],
        field_of_view: f64,
        super_samples_per_axis: Option<usize>,
        resolution: [usize; 2],
        num_tiles: [usize; 2],
    ) -> Self {
        let new = Self {
            position,
            target,
            field_of_view,
            super_samples_per_axis,
            resolution,
            num_tiles,
        };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the camera parameters are valid.
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
            && (self.super_samples_per_axis.is_none() || self.super_samples_per_axis.unwrap() > 0)
            && self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.num_tiles[0] > 0
            && self.num_tiles[1] > 0
    }

    /// Build a camera.
    pub fn build(&self) -> Camera {
        debug_assert!(self.is_valid());

        Camera::new(
            Point3::from_slice(&self.position),
            Point3::from_slice(&self.target),
            self.field_of_view.to_radians(),
            self.super_samples_per_axis.unwrap_or(1),
            self.resolution,
            self.num_tiles,
        )
    }
}
