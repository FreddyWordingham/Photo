use nalgebra::Point3;

use crate::{
    geometry::{Aabb, Ray},
    world::Instance,
};

const MAX_CHILDREN: usize = 2;

#[derive(Clone)]
struct InstanceBvhNode {
    pub aabb: Aabb,
    pub left_child: usize,
    pub count: usize,
}

pub struct InstanceBvh {
    indices: Vec<usize>,
    nodes: Vec<InstanceBvhNode>,
    nodes_used: usize,
}

impl InstanceBvh {
    pub fn new(instances: &[Instance]) -> Self {
        let instance_count = instances.len();

        let mut new = Self {
            indices: Vec::with_capacity(instance_count),
            nodes: Vec::with_capacity((instance_count * 2) - 1),
            nodes_used: 0,
        };
        new.build(instances);

        new
    }

    fn build(&mut self, instances: &[Instance]) {
        self.indices = (0..instances.len()).collect();
        self.nodes = vec![
            InstanceBvhNode {
                aabb: Aabb::new_unchecked(
                    Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                    Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
                ),
                left_child: 0,
                count: 0,
            };
            (instances.len() * 2) - 1
        ];
        self.nodes[0].left_child = 0;
        self.nodes[0].count = instances.len();
        self.nodes_used = 1;

        self.update_bounds(0, instances);
        self.subdivide(0, instances);

        self.nodes.truncate(self.nodes_used);
    }

    fn update_bounds(&mut self, index: usize, instances: &[Instance]) {
        for i in 0..self.nodes[index].count {
            let instance_aabb = &instances[self.indices[self.nodes[index].left_child + i]].aabb();
            self.nodes[index].aabb = self.nodes[index].aabb.union(instance_aabb);
        }
    }

    fn subdivide(&mut self, index: usize, instances: &[Instance]) {
        if self.nodes[index].count <= MAX_CHILDREN {
            return;
        }

        let extent = [
            self.nodes[index].aabb.maxs()[0] - self.nodes[index].aabb.mins()[0],
            self.nodes[index].aabb.maxs()[1] - self.nodes[index].aabb.mins()[1],
            self.nodes[index].aabb.maxs()[2] - self.nodes[index].aabb.mins()[2],
        ];
        let axis = if extent[0] > extent[1] && extent[0] > extent[2] {
            0
        } else if extent[1] > extent[2] {
            1
        } else {
            2
        };
        let split_position = self.nodes[index].aabb.mins()[axis] + (extent[axis] * 0.5);

        let mut i = self.nodes[index].left_child;
        let mut j = i + self.nodes[index].count - 1;

        while i <= j {
            if instances[self.indices[i]].aabb().centre()[axis] < split_position {
                i += 1;
            } else {
                let temp = self.indices[i];
                self.indices[i] = self.indices[j];
                self.indices[j] = temp;
                j -= 1;
            }
        }

        let left_count = i - self.nodes[index].left_child;
        if (left_count == 0) || (left_count == self.nodes[index].count) {
            return;
        }

        let left_child_index = self.nodes_used;
        self.nodes_used += 1;
        let right_child_index = self.nodes_used;
        self.nodes_used += 1;

        self.nodes[left_child_index].left_child = self.nodes[index].left_child;
        self.nodes[left_child_index].count = left_count;

        self.nodes[right_child_index].left_child = i;
        self.nodes[right_child_index].count = self.nodes[index].count - left_count;

        self.nodes[index].left_child = left_child_index;
        self.nodes[index].count = 0;

        self.update_bounds(left_child_index, instances);
        self.update_bounds(right_child_index, instances);
        self.subdivide(left_child_index, instances);
        self.subdivide(right_child_index, instances);
    }

    pub fn ray_intersect_indices(&self, ray: &Ray, instances: &[Instance]) -> Vec<usize> {
        let mut hits: Vec<(usize, f64)> = Vec::new();
        self.ray_intersect_node(0, ray, instances, &mut hits);
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        hits.into_iter().map(|(index, _)| index).collect()
    }

    fn ray_intersect_node(
        &self,
        node_index: usize,
        ray: &Ray,
        instances: &[Instance],
        hits: &mut Vec<(usize, f64)>,
    ) {
        if self.nodes[node_index].aabb.ray_intersect(ray) {
            if self.nodes[node_index].count == 0 {
                self.ray_intersect_node(self.nodes[node_index].left_child, ray, instances, hits);
                self.ray_intersect_node(
                    self.nodes[node_index].left_child + 1,
                    ray,
                    instances,
                    hits,
                );
            } else {
                for i in 0..self.nodes[node_index].count {
                    let instance_index = self.indices[self.nodes[node_index].left_child + i];
                    let instance_aabb = instances[instance_index].aabb();
                    if let Some(instance_distance) = instance_aabb.ray_intersect_distance(ray) {
                        hits.push((instance_index, instance_distance));
                    }
                }
            }
        }
    }
}
