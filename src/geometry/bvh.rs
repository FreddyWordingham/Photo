//! Bounding Volume Hierarchy node structure.

use crate::geometry::Aabb;

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
}

impl Bvh {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(indices: Vec<usize>, nodes: Vec<BvhNode>) -> Self {
        debug_assert!(
            !indices.is_empty(),
            "Bounding Volume Hierarchy must contain at least one object!"
        );
        debug_assert!(
            !nodes.is_empty(),
            "Bounding Volume Hierarchy must contain at least one node!"
        );

        Self { indices, nodes }
    }
}
