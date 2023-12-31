//! Ray structure.

use core::ops::Mul;

use nalgebra::{Point3, Similarity3, Unit, Vector3};

/// Line with a fixed starting location and direction.
#[derive(Clone)]
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

    /// Travel the origin along the [`Ray`]'s direction (meters).
    #[inline]
    pub fn travel(&mut self, distance: f64) {
        debug_assert!(distance.is_finite(), "Distance must be finite.");
        debug_assert!(distance >= 0.0, "Distance must be positive.");

        self.origin += self.direction.as_ref() * distance;
    }

    /// Reflect the direction about a normal.
    #[inline]
    pub fn reflect(&mut self, normal: Unit<Vector3<f64>>) {
        let i = self.direction.as_ref();
        let n = normal.as_ref();
        self.direction = Unit::new_normalize(i - 2.0 * i.dot(n) * n);
    }

    #[inline]
    pub fn refract(
        &mut self,
        normal: Unit<Vector3<f64>>,
        current_refractive_index: f64,
        next_refractive_index: f64,
    ) {
        let i = self.direction.as_ref();
        let n = normal.as_ref();
        let eta = current_refractive_index / next_refractive_index;
        let cos_theta_i = -i.dot(n);
        let cos_theta_t = (eta * eta)
            .mul_add(-cos_theta_i.mul_add(-cos_theta_i, 1.0), 1.0)
            .sqrt();
        self.direction = Unit::new_normalize(eta * i + eta.mul_add(cos_theta_i, -cos_theta_t) * n);
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
