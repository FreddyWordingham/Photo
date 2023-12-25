//! Bounding Volume Hierarchy node structure.

use crate::geometry::Aabb;

/// Bounding Volume Hierarchy node.
struct BvhNode {
    /// Bounding box.
    aabb: Aabb,
    /// Left child node index. Right child node index is `left_child + 1`.
    left_child: usize,
    /// Number of objects contained in this node.
    count: usize,
}

/// Bounding Volume Hierarchy.
pub struct Bvh {
    /// Indices of objects contained in this node.
    indices: Vec<usize>,
    /// List of nodes.
    nodes: Vec<BvhNode>,
    /// Total number of nodes.
    node_count: usize,
}
