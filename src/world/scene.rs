use nalgebra::Point3;
use palette::LinSrgba;

use crate::{
    assets::Resources,
    geometry::{Aabb, Ray},
    render::Sample,
    world::{Bvh, Instance},
};

pub struct Scene<'a> {
    _resources: &'a Resources,
    instances: Vec<Instance<'a>>,
    _instance_bvh: Bvh,
    aabb: Aabb,
}

impl<'a> Scene<'a> {
    pub fn new(resources: &'a Resources, instances: Vec<Instance<'a>>) -> Self {
        let aabb = Self::init_aabb(&instances);

        Self {
            _resources: resources,
            instances,
            _instance_bvh: Bvh {},
            aabb,
        }
    }

    fn init_aabb(instances: &[Instance]) -> Aabb {
        let empty = Aabb::new_unchecked(
            Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        );
        instances
            .iter()
            .fold(empty, |aabb, instance| aabb.union(&instance.aabb()))
    }

    pub fn sample(&self, pixel_index: [usize; 2], ray: Ray) -> Sample {
        if !self.aabb.intersect_ray(&ray) {
            return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
        }

        for instance in &self.instances {
            if instance.aabb().intersect_ray(&ray) {
                if instance.intersect_ray(&ray) {
                    let r = ray.direction().x.abs() as f32;
                    let g = ray.direction().y.abs() as f32;
                    let b = ray.direction().z.abs() as f32;
                    return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
                }
            }
        }

        return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
    }
}
