use palette::Srgba;

use crate::{geometry::Ray, world::Scene};

/// Sample the scene with a ray.
pub fn sample_scene(_scene: &Scene, ray: Ray) -> Srgba {
    let mut colour = Srgba::new(0.0, 0.0, 0.0, 1.0);

    colour.red = ray.direction.x.abs() as f32;
    colour.green = ray.direction.y.abs() as f32;
    colour.blue = ray.direction.z.abs() as f32;

    colour
}
