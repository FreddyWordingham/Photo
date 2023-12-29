//! Collision geometry trait.

use crate::geometry::Aabb;

/// Types implementing this type can be checked for collisions with an axis-aligned bounding box.
pub trait Bounded {
    /// Get the axis-aligned bounding box of the shape.
    #[must_use]
    fn aabb(&self) -> Aabb;
}

/// Types implementing this trait can return an array of [`Aabb`]'s, accesses by index.
pub trait IndexedBounds<T: Bounded> {
    fn indexed_aabb(&self, index: usize) -> Aabb;
}

impl<T: Bounded> IndexedBounds<T> for Vec<T> {
    #[inline]
    fn indexed_aabb(&self, index: usize) -> Aabb {
        self[index].aabb()
    }
}
