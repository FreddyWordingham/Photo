use crate::{
    geometry::Bvh,
    world::{Instance, Resources},
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
}
