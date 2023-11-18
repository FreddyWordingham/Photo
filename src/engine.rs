use palette::Srgba;

use crate::{geometry::Ray, world::Scene};

/// Sample the scene with a ray.
pub fn sample_scene(scene: &Scene, ray: Ray) -> Srgba {
    let mut colour = Srgba::new(0.0, 0.0, 0.0, 1.0);

    colour.red = ray.direction.x.abs() as f32;
    colour.green = ray.direction.y.abs() as f32;
    colour.blue = ray.direction.z.abs() as f32;

    for object in scene.objects.values() {
        if object.intersect_ray(&ray, scene.meshes.get(object.mesh_id()).unwrap()) {
            colour.red = 0.7;
            colour.green = 0.7;
            colour.blue = 0.7;
        }
    }

    colour
}
