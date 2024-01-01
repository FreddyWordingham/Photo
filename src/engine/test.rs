//! Test [`Ray`]-traced render engine function.

use core::f64::consts::{FRAC_PI_2, PI};

use nalgebra::{Point3, Unit, Vector3};
use palette::LinSrgba;

/// The golden ratio constant (2.0 / (1.0 + sqrt(5.0))).
const GOLDEN_RATIO: f64 = 1.618_033_988_749;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Material, Scene, Spectrum},
};

/// Test rendering [`Engine`] function.
///
/// # Panics
///
/// Panics if a [`Spectrum`] cannot be built.
#[must_use]
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    clippy::expect_used,
    clippy::min_ident_chars,
    clippy::needless_pass_by_value,
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

        let ambient = 1.0_f64;

        // Lightness
        let sun_direction = Unit::new_normalize(sun_position - contact_position);
        let diffuse = (contact.side * contact.smooth_normal.dot(&sun_direction)).max(0.0);

        // Darkness
        let shadow_cast_position =
            contact_position + (settings.smoothing_length * contact.side * contact.normal.as_ref());
        let mut shadow_ray = Ray::new(shadow_cast_position, sun_direction);
        let spectral = calculate_occlusion(settings, scene, &mut shadow_ray);

        let light_level =
            (spectral.mul_add(0.0, ambient.mul_add(1.0, diffuse * 0.0))).clamp(0.0, 1.0) as f32;
        let shadow_level = calculate_local_occlusion(
            settings,
            scene,
            shadow_cast_position,
            Unit::new_normalize(contact.side * contact.normal.as_ref()),
        );

        match contact.material {
            Material::Diffuse { spectrum } | Material::Reflective { spectrum, .. } => {
                let base_colour = spectrum.sample(light_level as f32);
                let illuminated_spectrum =
                    Spectrum::new(vec![LinSrgba::new(0.0, 0.0, 0.0, 1.0), base_colour])
                        .expect("Failed to build colour gradient!");
                return illuminated_spectrum.sample(shadow_level as f32);
            }
            Material::Refractive { spectrum, .. } => {
                return spectrum.sample(light_level as f32);
            }
        }
    }

    colour
}

/// Calculate the occlusion of a [`Ray`] through a [`Scene`].
#[must_use]
#[inline]
fn calculate_occlusion(settings: &Settings, scene: &Scene, shadow_ray: &mut Ray) -> f64 {
    let mut light = 1.0;
    while let Some(shadow_contact) = scene.ray_intersect_contact(shadow_ray) {
        match shadow_contact.material {
            Material::Diffuse { .. } => {
                light = 0.0;
            }
            Material::Reflective { absorption, .. } | Material::Refractive { absorption, .. } => {
                light *= 1.0 - absorption;
            }
        }

        shadow_ray.travel(shadow_contact.distance + settings.smoothing_length);

        if light < settings.min_weight {
            light = 0.0;
            break;
        }
    }

    light
}

/// Calculate the local occlusion of a `[Point3<f64>`] within a [`Scene`].
#[must_use]
#[inline]
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
#[must_use]
#[inline]
#[allow(clippy::modulo_arithmetic)]
pub fn rand_sphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(
        n < max,
        "The sample index must be less than the number of samples!"
    );
    debug_assert!(max > 0, "The number of samples must be positive!");

    let delta = f64::from(1 - max).mul_add(0.5, f64::from(n));
    let phi = ((2.0 * delta) / f64::from(max)).asin() + FRAC_PI_2;
    let theta = ((2.0 * PI) / GOLDEN_RATIO) * (delta % GOLDEN_RATIO);

    (phi, theta)
}

/// Sample points on a hemisphere's surface using the golden ratio.
#[must_use]
#[inline]
pub fn rand_hemisphere_point(n: i32, max: i32) -> (f64, f64) {
    debug_assert!(
        n < max,
        "The sample index must be less than the number of samples!"
    );
    debug_assert!(max > 0, "The number of samples must be positive!");

    rand_sphere_point(n, max * 2)
}
