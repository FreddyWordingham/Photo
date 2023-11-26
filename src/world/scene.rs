use crate::{
    geometry::Ray,
    world::{Instance, InstanceBvh},
};

pub struct Scene<'a> {
    bvh: InstanceBvh,
    instances: Vec<Instance<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(instances: Vec<Instance<'a>>) -> Self {
        Self {
            bvh: InstanceBvh::new(&instances),
            instances,
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        self.bvh.ray_intersect_indices(ray, &self.instances).len() > 0
    }
}
