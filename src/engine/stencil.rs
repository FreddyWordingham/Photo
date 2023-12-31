//! Stencil render engine function.

use palette::LinSrgba;

use crate::{geometry::Ray, render::Settings, world::Scene};

/// Stencil whether the [`Ray`] intersects with the [`Scene`].
#[must_use]
#[inline]
#[allow(
    clippy::cast_possible_truncation,
    clippy::min_ident_chars,
    clippy::needless_pass_by_value
)]
pub fn stencil(_settings: &Settings, scene: &Scene, ray: Ray) -> LinSrgba {
    if scene.ray_intersect(&ray) {
        LinSrgba::new(1.0, 1.0, 1.0, 1.0)
    } else {
        LinSrgba::new(0.0, 0.0, 0.0, 0.0)
    }
}
