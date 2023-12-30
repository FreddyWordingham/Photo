//! Diffuse lighting render engine function.

use std::time::Instant;

use nalgebra::{Point3, Unit};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Sample,
    world::{Material, Scene, Spectrum},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// colour blue for the outside, and red for the inside.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn side(
    scene: &Scene,
    pixel_index: [usize; 2],
    ray: &Ray,
    sun_position: &Point3<f64>,
    max_shadow_distance: f64,
) -> Sample {
    let start_time = Instant::now();

    let red = Spectrum::new(vec![
        LinSrgba::new(0.1, 0.0, 0.0, 1.0),
        LinSrgba::new(1.0, 0.0, 0.0, 1.0),
    ])
    .expect("Failed to build colour gradient!");
    let blue = Spectrum::new(vec![
        LinSrgba::new(0.0, 0.0, 0.1, 1.0),
        LinSrgba::new(0.0, 0.0, 1.0, 1.0),
    ])
    .expect("Failed to build colour gradient!");

    if let Some(contact) = scene.ray_intersect_contact(&ray) {
        let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;
        let sun_direction = Unit::new_normalize(sun_position - contact_position);
        let lightness = (contact.side * contact.smooth_normal.dot(&sun_direction)).max(0.0) as f32;

        let shadow_cast_position =
            contact_position + (0.0001 * contact.side * contact.normal.as_ref());
        let shadow_ray = Ray::new(shadow_cast_position, sun_direction);
        let occlusion = scene
            .ray_intersect_distance(&shadow_ray)
            .map(|distance| (1.0 - (distance / max_shadow_distance)).clamp(0.0, 1.0))
            .unwrap_or(0.0) as f32;

        match contact.material {
            Material::Diffuse { .. }
            | Material::Reflective { .. }
            | Material::Refractive { .. } => {
                let colour = if contact.side < 0.0 {
                    red.sample(lightness * (1.0 - occlusion))
                } else {
                    blue.sample(lightness * (1.0 - occlusion))
                };
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
