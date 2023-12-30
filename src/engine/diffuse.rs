//! Diffuse lighting render engine function.

use std::time::Instant;

use nalgebra::{Point3, Unit};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Sample,
    world::{Material, Scene},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// lighting the scene with a single sun light source, casting shadows.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn diffuse(
    scene: &Scene,
    pixel_index: [usize; 2],
    ray: &Ray,
    sun_position: &Point3<f64>,
    max_shadow_distance: f64,
) -> Sample {
    let start_time = Instant::now();

    if let Some(contact) = scene.ray_intersect_contact(&ray) {
        let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;
        let sun_direction = Unit::new_normalize(sun_position - contact_position);
        let lightness = (contact.smooth_normal.dot(&sun_direction) as f32).max(0.0);

        let shadow_cast_position = contact_position + (contact.normal.as_ref() * 0.0001);
        let shadow_ray = Ray::new(shadow_cast_position, sun_direction);

        let occlusion = scene
            .ray_intersect_distance(&shadow_ray)
            .map(|distance| (1.0 - (distance / max_shadow_distance)).clamp(0.0, 1.0))
            .unwrap_or(0.0) as f32;

        match contact.material {
            Material::Diffuse { spectrum }
            | Material::Reflective { spectrum, .. }
            | Material::Refractive { spectrum, .. } => {
                let colour = spectrum.sample(lightness * (1.0 - occlusion));
                Sample::new(pixel_index, colour, start_time.elapsed())
            }
        }
    } else {
        Sample::new(
            pixel_index,
            LinSrgba::new(0.0, 0.0, 0.0, 0.0),
            start_time.elapsed(),
        )
    }
}
