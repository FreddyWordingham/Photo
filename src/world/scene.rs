use palette::LinSrgba;

use crate::{
    assets::Resources,
    geometry::Ray,
    render::Sample,
    world::{Bvh, Instance},
};

pub struct Scene<'a> {
    _resources: &'a Resources,
    _instances: Vec<Instance<'a>>,
    _instance_bvh: Bvh,
}

impl<'a> Scene<'a> {
    pub fn new(resources: &'a Resources, instances: Vec<Instance<'a>>) -> Self {
        Self {
            _resources: resources,
            _instances: instances,
            _instance_bvh: Bvh {},
        }
    }

    pub fn sample(&self, ray: Ray) -> Sample {
        let r = ray.direction().x.abs() as f32;
        let g = ray.direction().y.abs() as f32;
        let b = ray.direction().z.abs() as f32;

        // println!("r: {}, g: {}, b: {}", r, g, b);

        Sample::new(LinSrgba::new(r, g, b, 1.0))
    }
}
