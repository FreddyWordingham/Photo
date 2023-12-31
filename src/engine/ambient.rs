//! Ambient lighting render engine function.

use nalgebra::{Point3, Unit};
use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Material, Scene},
};

/// Render the surface [`Material`] [`Spectrum`] when [`Ray`]s intersect with the [`Scene`],
/// lighting the scene with a single sun light source.
#[must_use]
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    clippy::min_ident_chars,
    clippy::needless_pass_by_value
)]
pub fn ambient(
    _settings: &Settings,
    scene: &Scene,
    ray: Ray,
    sun_position: &Point3<f64>,
) -> LinSrgba {
    scene.ray_intersect_contact(&ray).map_or_else(
        || LinSrgba::new(0.0, 0.0, 0.0, 0.0),
        |contact| {
            let contact_position = ray.origin() + ray.direction().as_ref() * contact.distance;
            let sun_direction = Unit::new_normalize(sun_position - contact_position);
            let lightness = (contact.smooth_normal.dot(&sun_direction) as f32).max(0.0);
            match contact.material {
                Material::Diffuse { spectrum }
                | Material::Reflective { spectrum, .. }
                | Material::Refractive { spectrum, .. } => spectrum.sample(lightness),
            }
        },
    )
}
