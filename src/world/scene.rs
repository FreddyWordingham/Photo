use crate::{
    geometry::Ray,
    render::Hit,
    world::{Instance, InstanceBvh},
};

pub struct Scene<'a> {
    bvh: InstanceBvh,
    instances: Vec<Instance<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(instances: Vec<Instance<'a>>, bvh_max_children: usize) -> Self {
        debug_assert!(bvh_max_children >= 2);

        Self {
            bvh: InstanceBvh::new(&instances, bvh_max_children),
            instances,
        }
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .into_iter()
            .any(|n| self.instances[n].ray_intersect(ray))
    }

    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .into_iter()
            .filter_map(|n| self.instances[n].ray_intersect_distance(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn ray_intersect_hit(&self, ray: &Ray) -> Option<Hit> {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .into_iter()
            .filter_map(|n| self.instances[n].ray_intersect_hit(ray))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}
