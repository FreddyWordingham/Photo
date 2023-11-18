use nalgebra::{Unit, Vector3};
use palette::Srgba;

use crate::{geometry::Ray, world::Scene};

/// Sample the scene with a ray.
pub fn sample_scene(scene: &Scene, ray: Ray) -> Srgba {
    let mut colour = Srgba::new(0.0, 0.0, 0.0, 1.0);

    colour.red = ray.direction.x.abs() as f32;
    colour.green = ray.direction.y.abs() as f32;
    colour.blue = ray.direction.z.abs() as f32;

    let mut closest: Option<(f64, Unit<Vector3<f64>>)> = None;

    for object in scene.objects.values() {
        if let Some((distance, normal)) =
            object.intersect_ray_distance_normal(&ray, scene.meshes.get(object.mesh_id()).unwrap())
        {
            if closest.is_none() || distance < closest.unwrap().0 {
                closest = Some((distance, normal));
            }
        }
    }

    if let Some((distance, normal)) = closest {
        colour = shade_normal(distance, normal);
    }

    colour
}

fn shade_normal(_distance: f64, normal: Unit<Vector3<f64>>) -> Srgba {
    let red = normal.x.abs() as f32;
    let green = normal.y.abs() as f32;
    let blue = normal.z.abs() as f32;

    Srgba::new(red, green, blue, 1.0)
}

fn _shade_distance(distance: f64, _normal: Unit<Vector3<f64>>) -> Srgba {
    let red = 1.0 - (distance as f32 / 40.0);
    let green = 1.0 - (distance as f32 / 40.0);
    let blue = 1.0 - (distance as f32 / 40.0);

    Srgba::new(red, green, blue, 1.0)
}
