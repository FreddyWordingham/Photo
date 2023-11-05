use nalgebra::{Unit, Vector3};
use std::fmt::{Display, Formatter, Result};

/// Converts between pixel and world coordinates.
pub struct Camera {
    /// The position of the camera in world coordinates.
    position: Vector3<f64>,
    /// Target position of the camera in world coordinates.
    target: Vector3<f64>,
    /// Horizontal field of view of the camera in degrees.
    field_of_view: f64,
    /// The resolution of the image in pixels. [rows, columns]
    resolution: [usize; 2],
    /// The resolution of each tile in pixels. [rows, columns]
    tile_resolution: [usize; 2],
}

impl Camera {
    /// Constructs a new camera instance.
    pub fn new(
        position: Vector3<f64>,
        target: Vector3<f64>,
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
                .fold(0.0, |acc, (p, t)| { acc + (p - t).abs() })
                > 0.0
        );
        debug_assert!(field_of_view > 0.0);
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
            && self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.tile_resolution[0] > 0
            && self.tile_resolution[1] > 0
            && self.resolution[0] % self.tile_resolution[0] == 0
            && self.resolution[1] % self.tile_resolution[1] == 0
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
}

impl Display for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "valid:                         {}", self.is_valid())?;

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
            "field of view:                 {} radians",
            self.field_of_view
        )?;

        writeln!(
            f,
            "resolution:                    [{}, {}] pixels",
            self.resolution[0], self.resolution[1]
        )?;

        write!(
            f,
            "tile resolution:               [{}, {}] pixels",
            self.tile_resolution[0], self.tile_resolution[1]
        )
    }
}
