//! Surface normal render engine function.

use std::time::Instant;

use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::{Sample, Settings},
    world::Scene,
};

/// Render the surface normals when [`Ray`]s intersect with the [`Scene`].
#[must_use]
#[inline]
#[allow(clippy::cast_possible_truncation, clippy::min_ident_chars)]
pub fn normal(_settings: &Settings, scene: &Scene, pixel_index: [usize; 2], ray: &Ray) -> Sample {
    let start_time = Instant::now();

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if let Some(contact) = scene.ray_intersect_contact(ray) {
        r = contact.normal.x.abs() as f32;
        g = contact.normal.y.abs() as f32;
        b = contact.normal.z.abs() as f32;
    };

    Sample::new(
        pixel_index,
        LinSrgba::new(r, g, b, 1.0),
        start_time.elapsed(),
    )
}
