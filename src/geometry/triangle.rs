use nalgebra::{Point3, Vector3};

use crate::geometry::Aabb;

pub struct Triangle {
    /// Vertices.
    pub vertex_positions: [Point3<f64>; 3],
}

impl Triangle {
    /// Construct a new instance.
    pub fn new(vertex_positions: [Point3<f64>; 3]) -> Self {
        Self { vertex_positions }
    }

    fn project_onto_axis(&self, axis: &Vector3<f64>) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for &vertex in &self.vertex_positions {
            let projection = vertex.coords.dot(axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }

    /// Check if the triangle intersects an AABB.
    pub fn intersects_aabb(&self, aabb: &Aabb) -> bool {
        // 1. Test for overlap on the box axes (X, Y, and Z axes)
        if !self.overlaps_on_box_axes(aabb) {
            return false;
        }

        // 2. Test for overlap on the triangle's normal axis
        let normal = self.triangle_normal();
        if !self.overlaps_on_axis(&normal, aabb) {
            return false;
        }

        // 3. Test for overlap on the nine edge cross-product axes
        let box_axes = [
            Vector3::new(1.0, 0.0, 0.0), // X-axis
            Vector3::new(0.0, 1.0, 0.0), // Y-axis
            Vector3::new(0.0, 0.0, 1.0), // Z-axis
        ];

        for i in 0..3 {
            for box_axis in &box_axes {
                let axis = self.edge_axis(i).cross(box_axis);
                if !self.overlaps_on_axis(&axis, aabb) {
                    return false;
                }
            }
        }

        true
    }

    fn overlaps_on_box_axes(&self, aabb: &Aabb) -> bool {
        // For each axis (X, Y, Z)
        for i in 0..3 {
            let axis = self.box_axis(i);

            // Project both the triangle and the AABB onto the axis
            let (min_tri, max_tri) = self.project_onto_axis(&axis);
            let (min_aabb, max_aabb) = aabb.project_onto_axis(&axis);

            // Check for overlap
            if max_tri < min_aabb || min_tri > max_aabb {
                return false;
            }
        }
        true
    }

    fn triangle_normal(&self) -> Vector3<f64> {
        let u = self.vertex_positions[1] - self.vertex_positions[0];
        let v = self.vertex_positions[2] - self.vertex_positions[0];
        u.cross(&v).normalize()
    }

    fn overlaps_on_axis(&self, axis: &Vector3<f64>, aabb: &Aabb) -> bool {
        let (min_tri, max_tri) = self.project_onto_axis(axis);
        let (min_aabb, max_aabb) = aabb.project_onto_axis(axis);

        !(max_tri < min_aabb || min_tri > max_aabb)
    }

    fn edge_axis(&self, index: usize) -> Vector3<f64> {
        self.vertex_positions[(index + 1) % 3] - self.vertex_positions[index]
    }

    fn box_axis(&self, index: usize) -> Vector3<f64> {
        match index {
            0 => Vector3::new(1.0, 0.0, 0.0),
            1 => Vector3::new(0.0, 1.0, 0.0),
            2 => Vector3::new(0.0, 0.0, 1.0),
            _ => panic!("Invalid index for box axis"),
        }
    }
}
