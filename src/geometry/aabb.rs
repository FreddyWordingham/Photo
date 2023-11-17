use nalgebra::{Point3, Vector3};

use crate::geometry::Ray;

pub struct Aabb {
    /// Minimum coordinates.
    pub mins: Point3<f64>,
    /// Maximum coordinates.
    pub maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    pub fn project_onto_axis(&self, axis: &Vector3<f64>) -> (f64, f64) {
        let corners = self.get_corners();
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for corner in corners {
            let projection = &corner.coords.dot(axis);
            min = min.min(*projection);
            max = max.max(*projection);
        }

        (min, max)
    }

    fn get_corners(&self) -> Vec<Point3<f64>> {
        let mut corners = Vec::with_capacity(8);

        // Iterate over all combinations of mins and maxs for x, y, and z
        for &x in &[self.mins.x, self.maxs.x] {
            for &y in &[self.mins.y, self.maxs.y] {
                for &z in &[self.mins.z, self.maxs.z] {
                    corners.push(Point3::new(x, y, z));
                }
            }
        }

        corners
    }

    pub fn intersects_aabb(&self, other: &Aabb) -> bool {
        // Check for separation along the x-axis
        if self.maxs.x < other.mins.x || other.maxs.x < self.mins.x {
            return false;
        }

        // Check for separation along the y-axis
        if self.maxs.y < other.mins.y || other.maxs.y < self.mins.y {
            return false;
        }

        // Check for separation along the z-axis
        if self.maxs.z < other.mins.z || other.maxs.z < self.mins.z {
            return false;
        }

        // No separation found, the AABBs intersect
        true
    }

    /// Test for an intersection distance with a ray.
    pub fn intersect_ray(&self, ray: &Ray) -> Option<f64> {
        let inv_direction = Vector3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t1 = (self.mins - ray.origin).component_mul(&inv_direction);
        let t2 = (self.maxs - ray.origin).component_mul(&inv_direction);

        let t_min = t1.zip_map(&t2, f64::min);
        let t_max = t1.zip_map(&t2, f64::max);

        let t_min = t_min.x.max(t_min.y).max(t_min.z);
        let t_max = t_max.x.min(t_max.y).min(t_max.z);

        if t_max < t_min || t_max < 0.0 {
            return None;
        }

        Some(t_min.max(0.0))
    }
}
