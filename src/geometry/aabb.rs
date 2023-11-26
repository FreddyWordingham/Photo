use nalgebra::{Point3, Vector3};

use crate::geometry::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    mins: Point3<f64>,
    maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        let new = Self { mins, maxs };

        debug_assert!(new.is_valid());

        new
    }

    pub fn new_unchecked(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    /// Check if the axis-aligned bounding box parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.mins < self.maxs
    }

    /// Get the minimum point.
    pub fn mins(&self) -> Point3<f64> {
        self.mins
    }

    /// Get the maximum point.
    pub fn maxs(&self) -> Point3<f64> {
        self.maxs
    }

    /// Get the center point.
    pub fn centre(&self) -> Point3<f64> {
        nalgebra::center(&self.mins, &self.maxs)
    }

    /// Get the corner points.
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

    pub fn project_onto_axis(&self, axis: &Vector3<f64>) -> (f64, f64) {
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

    pub fn overlaps_aabb(&self, other: &Aabb) -> bool {
        if self.maxs.x < other.mins.x || other.maxs.x < self.mins.x {
            return false;
        }
        if self.maxs.y < other.mins.y || other.maxs.y < self.mins.y {
            return false;
        }
        if self.maxs.z < other.mins.z || other.maxs.z < self.mins.z {
            return false;
        }
        true
    }

    /// Test for an intersection with a ray.
    pub fn intersect_ray(&self, ray: &Ray) -> bool {
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
    pub fn intersect_ray_distance(&self, ray: &Ray) -> Option<f64> {
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
    pub fn intersect_ray_position(&self, ray: &Ray) -> Option<Point3<f64>> {
        let distance = self.intersect_ray_distance(ray)?;
        Some(ray.origin() + distance * ray.direction().as_ref())
    }
}
