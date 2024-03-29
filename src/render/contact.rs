//! Surface contact point structure.

use nalgebra::{Unit, Vector3};

use crate::world::Material;

/// Surface intersection contact point.
#[non_exhaustive]
pub struct Contact<'a> {
    /// One if the contact point is on the outside of the surface, negative one if it is on the inside.
    pub side: f64,
    /// Distance to the contact point from the ray origin (meters).
    pub distance: f64,
    /// Flat normal of the surface at the contact point.
    pub normal: Unit<Vector3<f64>>,
    /// Smooth (interpolated) normal of the surface at the contact point.
    pub smooth_normal: Unit<Vector3<f64>>,
    /// Material of the surface.
    pub material: &'a Material<'a>,
}

impl<'a> Contact<'a> {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        is_inside: bool,
        distance: f64,
        normal: Unit<Vector3<f64>>,
        smooth_normal: Unit<Vector3<f64>>,
        material: &'a Material<'a>,
    ) -> Self {
        debug_assert!(distance.is_finite(), "Contact distance must be finite!");

        Self {
            side: if is_inside { -1.0 } else { 1.0 },
            distance,
            normal,
            smooth_normal,
            material,
        }
    }
}
