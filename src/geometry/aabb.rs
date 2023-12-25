//! Axis-aligned bounding box structure.

use nalgebra::Point3;

/// Axis-aligned bounding box.
pub struct Aabb {
    /// Minimum corner.
    min: Point3<f64>,
    /// Maximum corner.
    max: Point3<f64>,
}
