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
        self.bvh
            .ray_intersections(ray, &self.instances)
            .iter()
            .any(|&(n, _distance)| self.instances[n].ray_intersect(ray))
    }

    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .iter()
            .filter_map(|&(n, _distance)| self.instances[n].ray_intersect_distance(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}
