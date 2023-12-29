//! Bounding Volume Hierarchy builder structure.

use core::f64::{INFINITY, NEG_INFINITY};

use nalgebra::Point3;

use crate::geometry::{Aabb, Bounded, Bvh, BvhNode};

/// Builds a [`Bvh`] instance.
pub struct BvhBuilder {
    /// Indices of objects contained in this node.
    indices: Vec<usize>,
    /// List of nodes.
    nodes: Vec<BvhNode>,
    /// Current number of nodes used.
    nodes_used: usize,
}

impl BvhBuilder {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            nodes: Vec::new(),
            nodes_used: 0,
        }
    }

    /// Build a [`Bvh`] from a list of shapes.
    #[must_use]
    #[inline]
    pub fn build<T: Bounded>(mut self, shapes: &[T], max_children: usize, max_depth: usize) -> Bvh {
        debug_assert!(
            !shapes.is_empty(),
            "Bounding Volume Hierarchy must contain at least one shape!"
        );
        debug_assert!(
            max_children >= 2,
            "Mesh BVH max children must be greater than 2!"
        );
        debug_assert!(max_depth > 0, "Mesh BVH max depth must be positive!");

        self.indices = (0..shapes.len()).collect();
        self.nodes = vec![
            BvhNode {
                aabb: Aabb::new_unchecked(
                    Point3::new(INFINITY, INFINITY, INFINITY),
                    Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY),
                ),
                left_child: 0,
                count: 0,
            };
            (shapes.len() * 2) - 1
        ];

        self.nodes[0].left_child = 0;
        self.nodes[0].count = shapes.len();
        self.nodes_used = 1;

        self.update_bounds(0, shapes);
        self.subdivide(0, shapes, max_children, max_depth, 1);

        self.nodes.truncate(self.nodes_used);

        Bvh::new(self.indices, self.nodes)
    }

    /// Expand the bounding box of a node to include all objects contained in the node.
    #[inline]
    fn update_bounds<T: Bounded>(&mut self, index: usize, shapes: &[T]) {
        self.nodes[index].aabb = (0..self.nodes[index].count)
            .map(|i| shapes[self.indices[self.nodes[index].left_child + i]].aabb())
            .fold(self.nodes[index].aabb.clone(), |acc, aabb| acc.union(&aabb));
    }

    /// Subdivide a node into two child nodes if it contains more than `max_children` objects.
    #[inline]
    #[allow(clippy::print_stdout)]
    fn subdivide<T: Bounded>(
        &mut self,
        index: usize,
        shapes: &[T],
        max_children: usize,
        max_depth: usize,
        current_depth: usize,
    ) {
        debug_assert!(
            max_children >= 2,
            "BVH max children must be greater than 2!"
        );

        if (self.nodes[index].count <= max_children) || (current_depth > max_depth) {
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

        let split_position = extent[axis].mul_add(0.5, self.nodes[index].aabb.mins()[axis]);

        let mut i = self.nodes[index].left_child;
        let mut j = i + self.nodes[index].count - 1;

        while i <= j {
            if shapes[self.indices[i]].aabb().centre()[axis] < split_position {
                i += 1;
            } else {
                self.indices.swap(i, j);

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

        self.update_bounds(left_child_index, shapes);
        self.update_bounds(right_child_index, shapes);
        self.subdivide(
            left_child_index,
            shapes,
            max_children,
            max_depth,
            current_depth + 1,
        );
        self.subdivide(
            right_child_index,
            shapes,
            max_children,
            max_depth,
            current_depth + 1,
        );
    }
}

impl Default for BvhBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
