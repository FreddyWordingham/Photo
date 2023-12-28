//! Ray structure.

use nalgebra::{Point3, Unit, Vector3};

/// Line with a fixed starting location and direction.
pub struct Ray {
    /// Starting point.
    origin: Point3<f64>,
    /// Direction.
    direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Access the origin.
    #[must_use]
    #[inline]
    pub const fn origin(&self) -> Point3<f64> {
        self.origin
    }

    /// Access the direction.
    #[must_use]
    #[inline]
    pub const fn direction(&self) -> Unit<Vector3<f64>> {
        self.direction
    }
}
