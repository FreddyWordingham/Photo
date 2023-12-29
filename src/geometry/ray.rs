//! Ray structure.

use core::ops::Mul;

use nalgebra::{Point3, Similarity3, Unit, Vector3};

/// Line with a fixed starting location and direction.
pub struct Ray {
    /// Starting point.
    origin: Point3<f64>,
    /// Direction.
    direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Access the origin.
    #[must_use]
    #[inline]
    pub const fn origin(&self) -> Point3<f64> {
        self.origin
    }

    /// Access the direction.
    #[must_use]
    #[inline]
    pub const fn direction(&self) -> Unit<Vector3<f64>> {
        self.direction
    }
}

impl Mul<&Similarity3<f64>> for &Ray {
    type Output = Ray;

    /// Transform a [`Ray`] by a [`Similarity3`].
    #[must_use]
    #[inline]
    fn mul(self, transform: &Similarity3<f64>) -> Self::Output {
        Self::Output {
            origin: transform * self.origin,
            direction: Unit::new_normalize(transform * self.direction.as_ref()),
        }
    }
}
