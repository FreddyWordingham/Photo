use nalgebra::{Point3, Similarity3, Unit, Vector3};
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    pub fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Get the origin.
    pub fn origin(&self) -> &Point3<f64> {
        &self.origin
    }

    /// Get the direction.
    pub fn direction(&self) -> &Unit<Vector3<f64>> {
        &self.direction
    }

    /// Travel (move the origin) along the ray's direction.
    pub fn travel(&mut self, distance: f64) {
        self.origin += distance * self.direction.as_ref();
    }
}

impl Mul<&Similarity3<f64>> for &Ray {
    type Output = Ray;

    fn mul(self, transform: &Similarity3<f64>) -> Self::Output {
        Self::Output {
            origin: transform * self.origin,
            direction: Unit::new_normalize(transform * self.direction.as_ref()),
        }
    }
}
