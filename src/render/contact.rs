//! Surface contact point structure.

use nalgebra::{Unit, Vector3};

use crate::world::Material;

pub struct Contact<'a> {
    /// True if contact point is within the surface.
    is_inside: bool,
    /// Distance to the contact point from the ray origin.
    distance: f64,
    /// Flat normal of the surface at the contact point.
    normal: Unit<Vector3<f64>>,
    /// Smooth (interpolated) normal of the surface at the contact point.
    smooth_normal: Unit<Vector3<f64>>,
    /// Material of the surface.
    material: &'a Material<'a>,
}
