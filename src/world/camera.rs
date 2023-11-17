use nalgebra::{Matrix4, Point3, Unit, Vector3};
use std::{
    f64::consts::PI,
    fmt::{Display, Formatter, Result},
};

use crate::geometry::Ray;

/// Converts between pixel and world coordinates.
pub struct Camera {
    /// The position of the camera in world coordinates.
    position: Point3<f64>,
    /// Target position of the camera in world coordinates. [x, y, z] (meters)
    target: Point3<f64>,
    /// Horizontal field of view of the camera. (degrees)
    field_of_view: f64,
    // Super samples per axis.
    super_samples_per_axis: usize,
    /// The resolution of the image. [rows, columns] (tiles)
    image_resolution: [usize; 2],
    /// The resolution of each tile. [rows, columns] (pixels)
    tile_resolution: [usize; 2],
}

impl Camera {
    /// Constructs a new camera instance.
    pub fn new(
        position: Point3<f64>,
        target: Point3<f64>,
        field_of_view: f64,
        super_samples_per_axis: usize,
        image_resolution: [usize; 2],
        tile_resolution: [usize; 2],
    ) -> Self {
        debug_assert!(position.iter().all(|p| p.is_finite()));
        debug_assert!(target.iter().all(|t| t.is_finite()));
        debug_assert!(
            position
                .iter()
                .zip(target.iter())
                .fold(0.0, |acc, (p, t)| { acc + (p - t).abs() })
                > 0.0
        );
        debug_assert!(field_of_view > 0.0);
        debug_assert!(super_samples_per_axis > 0);
        debug_assert!(image_resolution[0] > 0);
        debug_assert!(image_resolution[1] > 0);
        debug_assert!(tile_resolution[0] > 0);
        debug_assert!(tile_resolution[1] > 0);

        Self {
            position,
            target,
            field_of_view,
            super_samples_per_axis,
            image_resolution,
            tile_resolution,
        }
    }

    /// Get the horizontal field of view of the camera. (radians)
    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    /// Get the aspect ratio of the camera.
    pub fn aspect_ratio(&self) -> f64 {
        (self.image_resolution[1] * self.tile_resolution[1]) as f64
            / (self.image_resolution[0] * self.tile_resolution[0]) as f64
    }

    /// Get the super samples per axis.
    pub fn super_samples_per_axis(&self) -> usize {
        self.super_samples_per_axis
    }

    /// Get the image resolution. [rows, columns] (tiles)
    pub fn image_resolution(&self) -> [usize; 2] {
        self.image_resolution
    }

    /// Get the tile resolution. [rows, columns]
    pub fn tile_resolution(&self) -> [usize; 2] {
        self.tile_resolution
    }

    /// Get the forwards direction of the camera.
    pub fn forwards(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.target - self.position)
    }

    /// Get the right direction of the camera.
    pub fn right(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(Vector3::z_axis().cross(&self.forwards()))
    }

    /// Get the up direction of the camera.
    pub fn up(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.forwards().cross(&self.right()))
    }

    /// Generate a ray from the camera to the given pixel.
    pub fn gen_ray(&self, x: f64, y: f64) -> Ray {
        debug_assert!(x >= 0.0);
        debug_assert!(x <= (self.image_resolution[0] * self.tile_resolution[0]) as f64);
        debug_assert!(y >= 0.0);
        debug_assert!(y <= (self.image_resolution[1] * self.tile_resolution[1]) as f64);

        let d_theta =
            self.field_of_view / (self.image_resolution[1] * self.tile_resolution[1]) as f64;
        let d_phi = (self.field_of_view / self.aspect_ratio())
            / (self.image_resolution[0] * self.tile_resolution[0]) as f64;

        let theta = (y * d_theta) - (self.field_of_view * 0.5);
        let phi = (x * d_phi) - (self.field_of_view / self.aspect_ratio() * 0.5);

        let forwards = self.forwards();
        let right = self.right();
        let up = self.up();

        let vertical_rotation = nalgebra::Rotation3::from_axis_angle(&up, phi);
        let lateral_rotation = nalgebra::Rotation3::from_axis_angle(&right, theta);

        let direction = lateral_rotation * vertical_rotation * forwards;

        Ray::new(self.position, direction)
    }

    /// Calculate the MVP matrix for the camera.
    /// This is the matrix that converts from world coordinates to screen coordinates.
    pub fn model_view_projection(&self) -> Matrix4<f64> {
        let view = Matrix4::look_at_rh(&self.position, &self.target, &Vector3::z_axis());
        let projection =
            Matrix4::new_perspective(self.aspect_ratio(), self.field_of_view, 0.1, 1000.0);

        projection * view
    }

    /// Calculate the inverse MVP matrix for the camera.
    /// This is the matrix that converts from screen coordinates to world coordinates.
    pub fn inverse_model_view_projection(&self) -> Matrix4<f64> {
        self.model_view_projection().try_inverse().unwrap()
    }
}

impl Display for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(
            f,
            "position:                      [{}, {}, {}] meters",
            self.position[0], self.position[1], self.position[2]
        )?;

        writeln!(
            f,
            "target:                        [{}, {}, {}] meters",
            self.target[0], self.target[1], self.target[2]
        )?;

        writeln!(
            f,
            "field of view:                 {} degrees",
            self.field_of_view * 180.0 / PI
        )?;

        writeln!(
            f,
            "image resolution:              [{}, {}] tiles",
            self.image_resolution[0], self.image_resolution[1]
        )?;

        writeln!(
            f,
            "tile resolution:               [{}, {}] pixels",
            self.tile_resolution[0], self.tile_resolution[1]
        )?;

        write!(
            f,
            "total resolution:              [{}, {}] pixels",
            self.tile_resolution[0] * self.image_resolution[0],
            self.tile_resolution[1] * self.image_resolution[1]
        )
    }
}
