//! Stencil render engine function.

use std::time::Instant;

use palette::LinSrgba;

use crate::{geometry::Ray, render::Sample, world::Scene};

/// Stencil whether the [`Ray`] intersects with the [`Scene`].
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn stencil(scene: &Scene, pixel_index: [usize; 2], ray: &Ray) -> Sample {
    let start_time = Instant::now();

    if scene.ray_intersect(ray) {
        Sample::new(
            pixel_index,
            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
            start_time.elapsed(),
        )
    } else {
        Sample::new(
            pixel_index,
            LinSrgba::new(0.0, 0.0, 0.0, 0.0),
            start_time.elapsed(),
        )
    }
}
