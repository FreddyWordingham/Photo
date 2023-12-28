//! Axis-aligned bounding box structure.

use nalgebra::{Point3, Unit, Vector3};

/// Axis-aligned bounding box.
#[derive(Clone)]
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

    /// Construct a new instance without checking the minimums are less than, or equal to, the maximums.
    #[must_use]
    #[inline]
    pub const fn new_unchecked(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Get the minimum corner.
    #[must_use]
    #[inline]
    pub const fn mins(&self) -> Point3<f64> {
        self.mins
    }

    /// Get the maximum corner.
    #[must_use]
    #[inline]
    pub const fn maxs(&self) -> Point3<f64> {
        self.maxs
    }

    /// Get the centre point.
    #[must_use]
    #[inline]
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Get the corner points.
    #[must_use]
    #[inline]
    pub fn corners(&self) -> Vec<Point3<f64>> {
        let mut corners = Vec::with_capacity(8);
        for &x in &[self.mins.x, self.maxs.x] {
            for &y in &[self.mins.y, self.maxs.y] {
                for &z in &[self.mins.z, self.maxs.z] {
                    corners.push(Point3::new(x, y, z));
                }
            }
        }
        corners
    }

    /// Find the union of two axis-aligned bounding boxes.
    #[must_use]
    #[inline]
    pub fn union(&self, other: &Self) -> Self {
        let mins = Point3::new(
            self.mins.x.min(other.mins.x),
            self.mins.y.min(other.mins.y),
            self.mins.z.min(other.mins.z),
        );
        let maxs = Point3::new(
            self.maxs.x.max(other.maxs.x),
            self.maxs.y.max(other.maxs.y),
            self.maxs.z.max(other.maxs.z),
        );

        Self::new(mins, maxs)
    }

    /// Check if two axis-aligned bounding boxes overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_aabb(&self, other: &Self) -> bool {
        [
            (self.maxs.x, other.mins.x, other.maxs.x, self.mins.x),
            (self.maxs.y, other.mins.y, other.maxs.y, self.mins.y),
            (self.maxs.z, other.mins.z, other.maxs.z, self.mins.z),
        ]
        .iter()
        .all(|&(max1, min2, max2, min1)| !(max1 < min2 || max2 < min1))
    }

    /// Find the minimum and maximum projections of the axis-aligned bounding box onto the given axis.
    #[must_use]
    #[inline]
    pub fn project_onto_axis(&self, axis: &Unit<Vector3<f64>>) -> (f64, f64) {
        let corners = self.corners();
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for corner in corners {
            let projection = &corner.coords.dot(axis);
            min = min.min(*projection);
            max = max.max(*projection);
        }

        (min, max)
    }
}
