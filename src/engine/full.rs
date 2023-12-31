//! Full [`Ray`]-traced render engine function.

use nalgebra::{Point3, Unit, Vector3};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Material, Scene},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// lighting the scene with a single sun light source, casting shadows with reflections.
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn full(
    settings: &Settings,
    scene: &Scene,
    mut ray: Ray,
    current_depth: u32,
    current_refractive_index: f64,
    mut weight: f64,
    sun_position: &Point3<f64>,
    max_shadow_distance: f64,
) -> LinSrgba {
    debug_assert!(
        current_refractive_index.is_finite(),
        "Current refractive index must be finite!"
    );
    debug_assert!(
        current_refractive_index >= 1.0,
        "Current refractive index must be at least 1.0!"
    );

    let mut colour = LinSrgba::new(0.0, 0.0, 0.0, 0.0);

    if current_depth <= settings.max_recursions && weight >= settings.min_weight {
        let mut loops = 0;
        while let Some(contact) = scene.ray_intersect_contact(&ray) {
            let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;

            // Lightness
            let sun_direction = Unit::new_normalize(sun_position - contact_position);
            let lightness = (contact.side * contact.smooth_normal.dot(&sun_direction)).max(0.0);

            // Darkness
            let shadow_cast_position = contact_position
                + (settings.smoothing_length * contact.side * contact.normal.as_ref());
            let mut shadow_ray = Ray::new(shadow_cast_position, sun_direction);
            let mut darkness = 1.0;
            while let Some(shadow_contact) = scene.ray_intersect_contact(&shadow_ray) {
                darkness *= 1.0 - shadow_contact.material.absorption();
                shadow_ray.travel(shadow_contact.distance + settings.smoothing_length);

                if darkness < settings.min_weight {
                    darkness = 0.0;
                    break;
                }
            }

            match contact.material {
                Material::Diffuse { spectrum } => {
                    colour += spectrum.sample((lightness * darkness) as f32) * weight as f32;
                    break;
                }
                Material::Reflective {
                    spectrum,
                    absorption,
                } => {
                    let surface_colour = spectrum.sample((lightness * darkness) as f32);
                    colour += surface_colour * (weight * absorption) as f32;
                    weight *= 1.0 - absorption;

                    ray.travel(contact.distance);
                    ray.reflect(contact.smooth_normal);
                    ray.travel(settings.smoothing_length);
                }
                Material::Refractive {
                    spectrum,
                    absorption,
                    refractive_index: next_refractive_index,
                } => {
                    let mut c_ref_index = current_refractive_index;
                    let mut n_ref_index = *next_refractive_index;
                    if contact.side < 0.0 {
                        std::mem::swap(&mut c_ref_index, &mut n_ref_index);
                    }

                    let surface_colour = spectrum.sample((lightness * darkness) as f32);
                    colour += surface_colour * (weight * absorption) as f32;

                    ray.travel(contact.distance);

                    let (reflection_prob, transmission_prob) = fresnel(
                        ray.direction(),
                        contact.smooth_normal,
                        c_ref_index,
                        n_ref_index,
                    );

                    let absorbed_weight = weight * absorption;
                    let remaining_weight = weight * (1.0 - absorption);
                    let reflected_weight = remaining_weight * reflection_prob;
                    let transmitted_weight = remaining_weight * transmission_prob;

                    let surface_colour = spectrum.sample((lightness * darkness) as f32);

                    let mut reflected_ray = ray.clone();
                    reflected_ray.reflect(contact.smooth_normal);
                    reflected_ray.travel(settings.smoothing_length);
                    let reflected_colour = full(
                        settings,
                        scene,
                        reflected_ray,
                        current_depth + 1,
                        c_ref_index,
                        reflected_weight,
                        sun_position,
                        max_shadow_distance,
                    );

                    let mut refracted_ray = ray.clone();
                    refracted_ray.refract(
                        Unit::new_normalize(contact.side * contact.smooth_normal.as_ref()),
                        c_ref_index,
                        n_ref_index,
                    );
                    refracted_ray.travel(settings.smoothing_length);
                    let refracted_colour = full(
                        settings,
                        scene,
                        refracted_ray,
                        current_depth + 1,
                        n_ref_index,
                        transmitted_weight,
                        sun_position,
                        max_shadow_distance,
                    );

                    colour += (surface_colour * absorbed_weight as f32) +  // Absorption
                    (reflected_colour * reflected_weight as f32) +     // Reflection
                    (refracted_colour * transmitted_weight as f32); // Transmission

                    break;
                }
            }

            loops += 1;
            if weight <= settings.min_weight || loops >= settings.max_loops {
                break;
            }
        }
    }

    colour
}

fn fresnel(
    incoming: Unit<Vector3<f64>>,
    normal: Unit<Vector3<f64>>,
    n1: f64,
    n2: f64,
) -> (f64, f64) {
    let cos_i = incoming.dot(&normal).abs();
    let sin_t2 = n1 * n1 * (1.0 - cos_i * cos_i) / (n2 * n2);

    if sin_t2 > 1.0 {
        // Total internal reflection
        return (1.0, 0.0);
    }

    let cos_t = (1.0 - sin_t2).sqrt();

    let rs = ((n2 * cos_i) - (n1 * cos_t)) / ((n2 * cos_i) + (n1 * cos_t));
    let rp = ((n1 * cos_i) - (n2 * cos_t)) / ((n1 * cos_i) + (n2 * cos_t));

    let reflection_prob = (rs * rs + rp * rp) / 2.0;
    let transmission_prob = 1.0 - reflection_prob;

    (reflection_prob, transmission_prob)
}
