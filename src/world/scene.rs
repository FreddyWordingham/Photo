use nalgebra::{Unit, Vector3};

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
            .any(|&(n, _aabb_distance)| self.instances[n].ray_intersect(ray))
    }

    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .iter()
            .filter_map(|&(n, _aabb_distance)| self.instances[n].ray_intersect_distance(ray))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn ray_intersect_distance_normal(&self, ray: &Ray) -> Option<(f64, Unit<Vector3<f64>>)> {
        self.bvh
            .ray_intersections(ray, &self.instances)
            .iter()
            .filter_map(|&(n, _aabb_distance)| {
                self.instances[n]
                    .ray_intersect_distance_normal(ray)
                    .map(|result| (n, result))
            })
            .min_by(|(_, (a_distance, _)), (_, (b_distance, _))| {
                a_distance.partial_cmp(&b_distance).unwrap()
            })
            .map(|(_, result)| result)
    }
}
