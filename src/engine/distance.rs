//! Distance render engine function.

use std::time::Instant;

use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::{Sample, Settings},
    world::Scene,
};

/// Render the distance travelled by [`Ray`]s that intersect with the [`Scene`].
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn distance(
    _settings: &Settings,
    scene: &Scene,
    pixel_index: [usize; 2],
    ray: &Ray,
    distance_normaliser: f64,
) -> Sample {
    debug_assert!(
        distance_normaliser.is_finite(),
        "Distance normaliser must be finite."
    );
    debug_assert!(
        distance_normaliser > 0.0,
        "Distance normaliser must be positive."
    );

    let start_time = Instant::now();

    let x = if let Some(distance) = scene.ray_intersect_distance(ray) {
        (distance / distance_normaliser) as f32
    } else {
        0.0
    }
    .clamp(0.0, 1.0);

    Sample::new(
        pixel_index,
        LinSrgba::new(x, x, x, 1.0),
        start_time.elapsed(),
    )
}
