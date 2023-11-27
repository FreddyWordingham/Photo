use nalgebra::Point3;

use crate::{
    assets::Mesh,
    geometry::{Aabb, Ray, Triangle},
};

#[derive(Clone)]
struct MeshBvhNode {
    pub aabb: Aabb,
    pub left_child: usize,
    pub count: usize,
}

pub struct MeshBvh {
    indices: Vec<usize>,
    nodes: Vec<MeshBvhNode>,
    nodes_used: usize,
}

impl MeshBvh {
    pub fn new(triangles: &[Triangle], max_children: usize) -> Self {
        debug_assert!(max_children >= 2);

        let triangle_count = triangles.len();

        let mut new = Self {
            indices: Vec::with_capacity(triangle_count),
            nodes: Vec::with_capacity((triangle_count * 2) - 1),
            nodes_used: 0,
        };
        new.build(triangles, max_children);

        new
    }

    fn build(&mut self, triangles: &[Triangle], max_children: usize) {
        debug_assert!(max_children >= 2);

        self.indices = (0..triangles.len()).collect();
        self.nodes = vec![
            MeshBvhNode {
                aabb: Aabb::new_unchecked(
                    Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                    Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
                ),
                left_child: 0,
                count: 0,
            };
            (triangles.len() * 2) - 1
        ];
        self.nodes[0].left_child = 0;
        self.nodes[0].count = triangles.len();
        self.nodes_used = 1;

        self.update_bounds(0, triangles);
        self.subdivide(0, triangles, max_children);

        self.nodes.truncate(self.nodes_used);
    }

    fn update_bounds(&mut self, index: usize, triangles: &[Triangle]) {
        for i in 0..self.nodes[index].count {
            let triangle_aabb = &triangles[self.indices[self.nodes[index].left_child + i]].aabb();
            self.nodes[index].aabb = self.nodes[index].aabb.union(triangle_aabb);
        }
    }

    fn subdivide(&mut self, index: usize, triangles: &[Triangle], max_children: usize) {
        debug_assert!(max_children >= 2);

        if self.nodes[index].count <= max_children {
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
            if triangles[self.indices[i]].aabb().centre()[axis] < split_position {
                i += 1;
            } else {
                let temp = self.indices[i];
                self.indices[i] = self.indices[j];
                self.indices[j] = temp;

                if j == 0 {
                    println!(
                        "MESH BVH WARNING j == 0, when count is {}",
                        self.nodes[index].count
                    );
                    return;
                }

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

        self.update_bounds(left_child_index, triangles);
        self.update_bounds(right_child_index, triangles);
        self.subdivide(left_child_index, triangles, max_children);
        self.subdivide(right_child_index, triangles, max_children);
    }

    pub fn ray_intersections(&self, ray: &Ray, mesh: &Mesh) -> Vec<(usize, f64)> {
        let mut hits: Vec<(usize, f64)> = Vec::new();
        self.ray_intersect_node(0, ray, mesh, &mut hits);
        hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        hits
    }

    fn ray_intersect_node(
        &self,
        node_index: usize,
        ray: &Ray,
        mesh: &Mesh,
        hits: &mut Vec<(usize, f64)>,
    ) {
        if self.nodes[node_index].aabb.ray_intersect(ray) {
            if self.nodes[node_index].count == 0 {
                self.ray_intersect_node(self.nodes[node_index].left_child, ray, mesh, hits);
                self.ray_intersect_node(self.nodes[node_index].left_child + 1, ray, mesh, hits);
            } else {
                for i in 0..self.nodes[node_index].count {
                    let triangle_index = self.indices[self.nodes[node_index].left_child + i];
                    let triangle_aabb = mesh.triangle(triangle_index).aabb();
                    if let Some(triangle_aabb_distance) = triangle_aabb.ray_intersect_distance(ray)
                    {
                        hits.push((triangle_index, triangle_aabb_distance));
                    }
                }
            }
        }
    }
}
