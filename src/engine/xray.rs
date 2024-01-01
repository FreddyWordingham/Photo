//! X-ray render engine function.

use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Settings,
    world::{Scene, Spectrum},
};

/// Colour by the number of times a [`Ray`] intersects with the [`Scene`].
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
    clippy::needless_pass_by_value
)]
pub fn xray(settings: &Settings, scene: &Scene, mut ray: Ray) -> LinSrgba {
    let colour_map = Spectrum::new(vec![
        LinSrgba::new(0.0, 0.0, 1.0, 1.0),
        LinSrgba::new(1.0, 0.0, 0.0, 1.0),
    ])
    .expect("Failed to build colour gradient!");

    let mut intersections = 0;
    while let Some(contact) = scene.ray_intersect_contact(&ray) {
        ray.travel(contact.distance + settings.smoothing_length);
        intersections += 1;
    }

    if intersections > 0 {
        colour_map.sample(intersections as f32 / 10.0)
    } else {
        LinSrgba::new(0.0, 0.0, 0.0, 0.0)
    }
}
