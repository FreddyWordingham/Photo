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

    let x = if scene.ray_intersect(ray) { 1.0 } else { 0.0 };

    Sample::new(
        pixel_index,
        LinSrgba::new(x, x, x, 1.0),
        start_time.elapsed(),
    )
}
