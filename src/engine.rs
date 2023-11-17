use palette::Srgba;

use crate::{geometry::Ray, world::Scene};

/// Sample the scene with a ray.
pub fn sample_scene(scene: &Scene, ray: Ray) -> Srgba {
    let mut colour = Srgba::new(0.0, 0.0, 0.0, 1.0);

    colour.red = ray.direction.x.abs() as f32;
    colour.green = ray.direction.y.abs() as f32;
    colour.blue = ray.direction.z.abs() as f32;

    let scale = 1.0;
    for mesh in scene.meshes.values() {
        if let Some(hit) = mesh.intersect_ray(&ray) {
            colour.red = (scale * hit.x as f32).abs();
            colour.green = (scale * hit.y as f32).abs();
            colour.blue = (scale * hit.z as f32).abs();
        }
    }

    colour
}
