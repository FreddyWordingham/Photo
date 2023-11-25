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

    /// Check if the axis-aligned bounding box parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.mins < self.maxs
    }

    pub fn get_corners(&self) -> Vec<Point3<f64>> {
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

    /// Test for an intersection with a ray.
    pub fn intersect_ray(&self, _ray: &Ray) -> bool {
        todo!()
    }
}
