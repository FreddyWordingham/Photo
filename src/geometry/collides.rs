//! Collision geometry trait.

use crate::geometry::Aabb;

/// Types implementing this type can be checked for collisions with an axis-aligned bounding box.
pub trait Collides {
    /// Check if the object intersects with the given axis-aligned bounding box.
    #[must_use]
    fn intersect(&self, aabb: &Aabb) -> bool;

    /// Get the axis-aligned bounding box of the object.
    #[must_use]
    fn aabb(&self) -> Aabb;
}
