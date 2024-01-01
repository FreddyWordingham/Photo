//! Test [`Ray`]-traced render engine function.

use std::f64::consts::{FRAC_PI_2, PI};

use nalgebra::{Point3, Unit, Vector3};
use palette::LinSrgba;

const GOLDEN_RATIO: f64 = 1.618033988749;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Material, Scene, Spectrum},
};

/// Test rendering [`Engine`] function.
#[must_use]
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    clippy::min_ident_chars,
    clippy::too_many_lines
)]
pub fn test(
    settings: &Settings,
    scene: &Scene,
    ray: Ray,
    _current_depth: u32,
    current_refractive_index: f64,
    _weight: f64,
    sun_position: &Point3<f64>,
) -> LinSrgba {
    debug_assert!(
        current_refractive_index.is_finite(),
        "Current refractive index must be finite!"
    );
    debug_assert!(
        current_refractive_index >= 1.0,
        "Current refractive index must be at least 1.0!"
    );

    let colour = LinSrgba::new(1.0, 1.0, 1.0, 0.0);

    if let Some(contact) = scene.ray_intersect_contact(&ray) {
        let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;

        let ambient = 1.0;

        // Lightness
        let sun_direction = Unit::new_normalize(sun_position - contact_position);
        let diffuse = (contact.side * contact.smooth_normal.dot(&sun_direction)).max(0.0);

        // Darkness
        let shadow_cast_position =
            contact_position + (settings.smoothing_length * contact.side * contact.normal.as_ref());
        let mut shadow_ray = Ray::new(shadow_cast_position, sun_direction);
        let spectral = calculate_occlusion(settings, scene, &mut shadow_ray);

        let light_level =
            ((ambient * 0.1) + (diffuse * 0.2) + (spectral * 0.7)).clamp(0.0, 1.0) as f32;
        let shadow_level = calculate_local_occlusion(
            settings,
            scene,
            shadow_cast_position,
            Unit::new_normalize(contact.side * contact.normal.as_ref()),
        );

        match contact.material {
            Material::Diffuse { spectrum }
            | Material::Reflective { spectrum, .. }
            | Material::Refractive { spectrum, .. } => {
                let base_colour = spectrum.sample(light_level as f32);
                let spectrum =
                    Spectrum::new(vec![LinSrgba::new(0.0, 0.0, 0.0, 1.0), base_colour]).unwrap();
                return spectrum.sample(shadow_level as f32);
            }
        }
    }

    colour
}

fn calculate_occlusion(settings: &Settings, scene: &Scene, shadow_ray: &mut Ray) -> f64 {
    let mut light = 1.0;
    while let Some(shadow_contact) = scene.ray_intersect_contact(shadow_ray) {
        light *= 1.0 - shadow_contact.material.absorption();
        shadow_ray.travel(shadow_contact.distance + settings.smoothing_length);

        if light < settings.min_weight {
            light = 0.0;
            break;
        }
    }

    light
}

fn calculate_local_occlusion(
    settings: &Settings,
    scene: &Scene,
    shadow_cast_position: Point3<f64>,
    surface_normal: Unit<Vector3<f64>>,
) -> f64 {
    let samples = 101;

    let mut occlusion = 0.0;
    for n in 0..samples {
        let (phi, theta) = rand_hemisphere_point(n, samples);
        let mut ray = Ray::new(shadow_cast_position, surface_normal);
        ray.rotate(phi, theta);
        occlusion += calculate_occlusion(settings, scene, &mut ray);
    }

    occlusion / f64::from(samples)
}

/// Sample points on a sphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn rand_sphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    let d = f64::from(1 - max).mul_add(0.5, f64::from(n));
    let phi = ((2.0 * d) / f64::from(max)).asin() + FRAC_PI_2;
    let theta = ((2.0 * PI) / GOLDEN_RATIO) * (d % GOLDEN_RATIO);

    (phi, theta)
}

/// Sample points on a hemisphere's surface using the golden ratio.
#[inline]
#[must_use]
pub fn rand_hemisphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(n >= 0);
    debug_assert!(n < max);

    rand_sphere_point(n, max * 2)
}
