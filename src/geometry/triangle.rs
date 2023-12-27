//! Smooth triangle structure.

use nalgebra::{Point3, Unit, Vector3};

/// Three-dimensional triangle with interpolated surface normals.
pub struct Triangle {
    /// Vertex positions.
    vertex_positions: [Point3<f64>; 3],
    /// Vertex normals.
    vertex_normals: [Unit<Vector3<f64>>; 3],
}

impl Triangle {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(
        vertex_positions: [Point3<f64>; 3],
        vertex_normals: [Unit<Vector3<f64>>; 3],
    ) -> Self {
        Self {
            vertex_positions,
            vertex_normals,
        }
    }

    /// Access the vertex positions.
    #[must_use]
    #[inline]
    pub const fn vertex_positions(&self) -> &[Point3<f64>; 3] {
        &self.vertex_positions
    }

    /// Access the vertex normals.
    #[must_use]
    #[inline]
    pub const fn vertex_normals(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.vertex_normals
    }
}
