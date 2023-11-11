use nalgebra::{Point3, Unit, Vector3};
use std::f64::EPSILON;

use crate::geometry::Triangle;

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
}

pub struct Ray {
    /// Origin position.
    pub origin: Point3<f64>,
    /// Origin direction.
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    pub fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Travel (move the origin) along the ray's direction.
    pub fn travel(&mut self, distance: f64) {
        self.origin += distance * self.direction.as_ref();
    }

    /// Test for an intersection point with a triangle.
    pub fn intersect_triangle(&self, triangle: &Triangle) -> Option<Point3<f64>> {
        let edge1 = triangle.vertex_positions[1] - triangle.vertex_positions[0];
        let edge2 = triangle.vertex_positions[2] - triangle.vertex_positions[0];
        let h = self.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = self.origin - triangle.vertex_positions[0];
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * self.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > EPSILON {
            return Some(self.origin + t * self.direction.as_ref());
        }

        None
    }

    /// Test for an intersection distance with an AABB.
    pub fn intersect_aabb(&self, aabb: &Aabb) -> Option<f64> {
        let inv_direction = Vector3::new(
            1.0 / self.direction.x,
            1.0 / self.direction.y,
            1.0 / self.direction.z,
        );

        let t1 = (aabb.mins - self.origin).component_mul(&inv_direction);
        let t2 = (aabb.maxs - self.origin).component_mul(&inv_direction);

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
