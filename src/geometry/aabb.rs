//! Axis-aligned bounding box structure.

use nalgebra::Point3;

/// Axis-aligned bounding box.
pub struct Aabb {
    /// Minimum corner [x, y, z] (meters).
    min: Point3<f64>,
    /// Maximum corner [x, y, z] (meters).
    max: Point3<f64>,
}
