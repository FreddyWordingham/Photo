use crate::{
    assets::Resources,
    world::{Bvh, Instance},
};

pub struct Scene<'a> {
    _resources: &'a Resources,
    _instances: Vec<Instance<'a>>,
    _instance_bvh: Bvh,
}

impl<'a> Scene<'a> {
    pub fn new(resources: &'a Resources) -> Self {
        Self {
            _resources: resources,
            _instances: vec![],
            _instance_bvh: Bvh {},
        }
    }
}
