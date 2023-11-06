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
    /// The resolution of the image. [rows, columns] (tiles)
    image_resolution: [usize; 2],
    /// The resolution of each tile. [rows, columns] (pixels)
    tile_resolution: [usize; 2],
}

impl CameraBuilder {
    /// Construct a new instance.
    pub fn new(
        position: [f64; 3],
        target: [f64; 3],
        field_of_view: f64,
        image_resolution: [usize; 2],
        tile_resolution: [usize; 2],
    ) -> Self {
        let camera_builder = Self {
            position,
            target,
            field_of_view,
            image_resolution,
            tile_resolution,
        };

        debug_assert!(camera_builder.is_valid());

        camera_builder
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        if !self.position.iter().all(|p| p.is_finite()) {
            println!("INVALID! Invalid position: {:?}", self.position);
            return false;
        }

        if !self.target.iter().all(|t| t.is_finite()) {
            println!("INVALID! Invalid target: {:?}", self.target);
            return false;
        }

        if self
            .position
            .iter()
            .zip(self.target.iter())
            .fold(0.0, |acc, (p, t)| acc + (p - t).abs())
            == 0.0
        {
            println!(
                "INVALID! Position and target are the same: {:?}",
                self.position
            );
            return false;
        }

        if !(self.field_of_view > 0.0 && self.field_of_view < 180.0) {
            println!("INVALID! Invalid field of view: {:?}", self.field_of_view);
            return false;
        }

        if self.image_resolution[0] <= 0 {
            println!(
                "INVALID! Invalid vertical resolution: {:?}",
                self.image_resolution
            );
            return false;
        }

        if self.image_resolution[1] <= 0 {
            println!(
                "INVALID! Invalid horizontal resolution: {:?}",
                self.image_resolution
            );
            return false;
        }

        if self.tile_resolution[0] <= 0 {
            println!(
                "INVALID! Invalid vertical tile resolution: {:?}",
                self.tile_resolution[0]
            );
            return false;
        }

        if self.tile_resolution[1] <= 0 {
            println!(
                "INVALID! Invalid horizontal tile resolution: {}",
                self.tile_resolution[1]
            );
            return false;
        }

        true
    }

    /// Build a camera from the current settings.
    pub fn build(&self) -> Camera {
        debug_assert!(self.is_valid());

        Camera::new(
            Vector3::from_row_slice(&self.position),
            Vector3::from_row_slice(&self.target),
            self.field_of_view * PI / 180.0,
            self.image_resolution,
            self.tile_resolution,
        )
    }
}
