use palette::LinSrgba;

use crate::{
    geometry::Ray,
    render::Sample,
    world::{Bvh, Instance},
};

pub struct Scene<'a> {
    instance_bvh: Bvh,
    instances: Vec<Instance<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(instances: Vec<Instance<'a>>) -> Self {
        Self {
            instance_bvh: Bvh::new(&instances),
            instances,
        }
    }

    pub fn sample(&self, pixel_index: [usize; 2], ray: Ray) -> Sample {
        for i in self
            .instance_bvh
            .ray_intersect_indices(&ray, &self.instances)
        {
            let instance = &self.instances[i];
            if instance.intersect_ray(&ray) {
                let r = ray.direction().x.abs() as f32;
                let g = ray.direction().y.abs() as f32;
                let b = ray.direction().z.abs() as f32;
                return Sample::new(pixel_index, LinSrgba::new(r, g, b, 1.0));
            }
        }

        return Sample::new(pixel_index, LinSrgba::new(0.0, 0.0, 0.0, 0.0));
    }
}
