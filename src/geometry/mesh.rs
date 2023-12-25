//! Triangle mesh structure.

use nalgebra::{Point3, Unit, Vector3};

use crate::geometry::Bvh;

/// Triangular face.
struct Face {
    /// Vertex position indices.
    position_indices: [usize; 3],
    /// Vertex normal indices.
    normal_indices: [usize; 3],
}

/// Triangle mesh.
pub struct Mesh {
    /// Vertex positions.
    vertex_positions: Vec<Point3<f64>>,
    /// Vertex normals.
    vertex_normals: Vec<Unit<Vector3<f64>>>,
    /// List of faces.
    faces: Vec<Face>,
    /// Bounding Volume Hierarchy.
    bvh: Bvh,
}
