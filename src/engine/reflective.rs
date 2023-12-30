//! Reflective lighting render engine function.

use std::time::Instant;

use nalgebra::{Point3, Unit};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::{Sample, Settings},
    world::{Material, Scene},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// lighting the scene with a single sun light source, casting shadows with reflections.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn reflective(
    settings: &Settings,
    scene: &Scene,
    pixel_index: [usize; 2],
    mut ray: Ray,
    sun_position: &Point3<f64>,
    max_shadow_distance: f64,
) -> Sample {
    let start_time = Instant::now();
    let mut sample = Sample::new(
        pixel_index,
        LinSrgba::new(0.0, 0.0, 0.0, 0.0),
        start_time.elapsed(),
    );

    let mut weight = 1.0;
    let mut loops = 0;
    while let Some(contact) = scene.ray_intersect_contact(&ray) {
        if weight <= settings.min_weight || loops >= settings.max_loops {
            break;
        }

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
            Material::Reflective {
                spectrum,
                absorption,
            } => {
                ray.travel(contact.distance);
                ray.reflect(contact.smooth_normal);
                ray.travel(settings.smoothing_length);

                let surface_colour = spectrum.sample(lightness * (1.0 - occlusion));
                sample.colour += surface_colour * (weight * absorption) as f32;
                weight *= 1.0 - absorption;
            }
            Material::Diffuse { spectrum } | Material::Refractive { spectrum, .. } => {
                sample.colour += spectrum.sample(lightness * (1.0 - occlusion)) * weight as f32;
                break;
            }
        }

        loops += 1;
    }

    sample.time = start_time.elapsed();
    sample
}
