//! Distance render engine function.

use palette::LinSrgba;

use crate::{geometry::Ray, render::Settings, world::Scene};

/// Render the distance travelled by [`Ray`]s that intersect with the [`Scene`].
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn distance(
    _settings: &Settings,
    scene: &Scene,
    ray: Ray,
    distance_normaliser: f64,
) -> LinSrgba {
    debug_assert!(
        distance_normaliser.is_finite(),
        "Distance normaliser must be finite."
    );
    debug_assert!(
        distance_normaliser > 0.0,
        "Distance normaliser must be positive."
    );

    let x = if let Some(distance) = scene.ray_intersect_distance(&ray) {
        (distance / distance_normaliser) as f32
    } else {
        0.0
    }
    .clamp(0.0, 1.0);

    LinSrgba::new(x, x, x, 1.0)
}
