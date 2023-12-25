//! Ray structure.

use nalgebra::{Point3, Unit, Vector3};

/// Line with a fixed starting location and direction.
pub struct Ray {
    /// Starting point.
    origin: Point3<f64>,
    /// Direction.
    direction: Unit<Vector3<f64>>,
}
