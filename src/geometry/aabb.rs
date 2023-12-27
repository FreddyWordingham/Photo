//! Axis-aligned bounding box structure.

use nalgebra::Point3;

/// Axis-aligned bounding box.
pub struct Aabb {
    /// Minimum corner [x, y, z] (meters).
    mins: Point3<f64>,
    /// Maximum corner [x, y, z] (meters).
    maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        debug_assert!(
            mins <= maxs,
            "Axis-aligned bounding box minimums must be less than, or equal to, the maximums!"
        );

        Self { mins, maxs }
    }
}
