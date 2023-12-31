//! Bounding Volume Hierarchy node structure.

use crate::geometry::{Aabb, Bounded, IndexedBounds, Ray};

/// Bounding volume hierarchy node.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
pub struct BvhNode {
    /// Bounding box.
    pub aabb: Aabb,
    /// Left child node index. Right child node index is `left_child + 1`.
    pub left_child: usize,
    /// Number of objects contained in this node.
    pub count: usize,
}

/// Bounding volume hierarchy.
pub struct Bvh {
    /// Indices of objects contained in this node.
    indices: Vec<usize>,
    /// List of nodes.
    nodes: Vec<BvhNode>,
    /// Depth of the tree.
    depth: usize,
}

impl Bvh {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(indices: Vec<usize>, nodes: Vec<BvhNode>, depth: usize) -> Self {
        debug_assert!(
            !indices.is_empty(),
            "Bounding Volume Hierarchy must contain at least one object!"
        );
        debug_assert!(
            !nodes.is_empty(),
            "Bounding Volume Hierarchy must contain at least one node!"
        );

        Self {
            indices,
            nodes,
            depth,
        }
    }

    /// Check for a [`Ray`] intersection.
    ///
    /// # Panics
    ///
    /// If the comparison between intersection distances fails.
    #[must_use]
    #[inline]
    #[allow(clippy::unwrap_used)]
    pub fn ray_intersections<T: Bounded, S: IndexedBounds<T>>(
        &self,
        ray: &Ray,
        shapes: &S,
    ) -> Vec<(usize, f64)> {
        let mut hits = Vec::new();
        self.ray_intersect_node(0, ray, shapes, &mut hits);
        hits.sort_by(|distance_a, distance_b| distance_a.1.partial_cmp(&distance_b.1).unwrap());
        hits
    }

    /// Perform a [`Ray`] intersection with a [`BvhNode`].
    #[inline]
    fn ray_intersect_node<T: Bounded, S: IndexedBounds<T>>(
        &self,
        node_index: usize,
        ray: &Ray,
        shapes: &S,
        hits: &mut Vec<(usize, f64)>,
    ) {
        if self.nodes[node_index].aabb.ray_intersect(ray) {
            if self.nodes[node_index].count == 0 {
                self.ray_intersect_node(self.nodes[node_index].left_child, ray, shapes, hits);
                self.ray_intersect_node(self.nodes[node_index].left_child + 1, ray, shapes, hits);
            } else {
                for i in 0..self.nodes[node_index].count {
                    let index = self.indices[self.nodes[node_index].left_child + i];
                    let aabb = shapes.indexed_aabb(index);
                    if let Some(aabb_distance) = aabb.ray_intersect_distance(ray) {
                        hits.push((index, aabb_distance));
                    }
                }
            }
        }
    }
}
