//! Surface contact point structure.

use nalgebra::{Unit, Vector3};

use crate::world::Material;

/// Surface intersection contact point.
pub struct Contact<'a> {
    /// True if contact point is within the surface.
    is_inside: bool,
    /// Distance to the contact point from the ray origin (meters).
    distance: f64,
    /// Flat normal of the surface at the contact point.
    normal: Unit<Vector3<f64>>,
    /// Smooth (interpolated) normal of the surface at the contact point.
    smooth_normal: Unit<Vector3<f64>>,
    /// Material of the surface.
    material: &'a Material<'a>,
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
            is_inside,
            distance,
            normal,
            smooth_normal,
            material,
        }
    }
}
