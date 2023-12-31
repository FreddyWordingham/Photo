//! Diffuse lighting render engine function.

use nalgebra::{Point3, Unit};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Material, Scene},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// lighting the scene with a single sun light source, casting shadows.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn diffuse(
    settings: &Settings,
    scene: &Scene,
    ray: Ray,
    sun_position: &Point3<f64>,
    max_shadow_distance: f64,
) -> LinSrgba {
    if let Some(contact) = scene.ray_intersect_contact(&ray) {
        let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;
        let sun_direction = Unit::new_normalize(sun_position - contact_position);
        let lightness = (contact.side * contact.smooth_normal.dot(&sun_direction)).max(0.0) as f32;

        let shadow_cast_position =
            contact_position + (settings.smoothing_length * contact.side * contact.normal.as_ref());
        let shadow_ray = Ray::new(shadow_cast_position, sun_direction);
        let occlusion = scene
            .ray_intersect_distance(&shadow_ray)
            .map(|distance| (1.0 - (distance / max_shadow_distance)).clamp(0.0, 1.0))
            .unwrap_or(0.0) as f32;

        match contact.material {
            Material::Diffuse { spectrum }
            | Material::Reflective { spectrum, .. }
            | Material::Refractive { spectrum, .. } => {
                spectrum.sample(lightness * (1.0 - occlusion))
            }
        }
    } else {
        LinSrgba::new(0.0, 0.0, 0.0, 0.0)
    }
}
