//! Smooth triangle structure.

use nalgebra::{Point3, Unit, Vector3};

/// Three-dimensional triangle with interpolated surface normals.
pub struct Triangle {
    /// Vertex positions.
    vertex_positions: [Point3<f64>; 3],
    /// Vertex normals.
    vertex_normals: [Unit<Vector3<f64>>; 3],
}
