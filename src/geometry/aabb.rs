//! Axis-aligned bounding box structure.

use nalgebra::{Point3, Unit, Vector3};

use crate::geometry::Ray;

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

    /// Test for an intersection with a ray.
    #[must_use]
    #[inline]
    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        let inv_direction = Vector3::new(
            1.0 / ray.direction().x,
            1.0 / ray.direction().y,
            1.0 / ray.direction().z,
        );

        let t1 = (self.mins - ray.origin()).component_mul(&inv_direction);
        let t2 = (self.maxs - ray.origin()).component_mul(&inv_direction);

        let t_min = t1.zip_map(&t2, f64::min);
        let t_max = t1.zip_map(&t2, f64::max);

        let t_min = t_min.x.max(t_min.y).max(t_min.z);
        let t_max = t_max.x.min(t_max.y).min(t_max.z);

        !(t_max < t_min || t_max < 0.0)
    }

    /// Test for an intersection distance with a ray.
    /// Returns the distance along the ray to the intersection point.
    #[must_use]
    #[inline]
    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        let inv_direction = Vector3::new(
            1.0 / ray.direction().x,
            1.0 / ray.direction().y,
            1.0 / ray.direction().z,
        );

        let t1 = (self.mins - ray.origin()).component_mul(&inv_direction);
        let t2 = (self.maxs - ray.origin()).component_mul(&inv_direction);

        let t_min = t1.zip_map(&t2, f64::min);
        let t_max = t1.zip_map(&t2, f64::max);

        let t_min = t_min.x.max(t_min.y).max(t_min.z);
        let t_max = t_max.x.min(t_max.y).min(t_max.z);

        if t_max < t_min || t_max < 0.0 {
            return None;
        }

        Some(t_min.max(0.0))
    }

    /// Get the point of intersection with a ray.
    #[must_use]
    #[inline]
    pub fn ray_intersect_position(&self, ray: &Ray) -> Option<Point3<f64>> {
        let distance = self.ray_intersect_distance(ray)?;
        Some(ray.origin() + distance * ray.direction().as_ref())
    }
}
