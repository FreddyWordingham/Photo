use crate::geometry::AABB;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    positions: [[f32; 3]; 3],
    _coordinates: [[f32; 2]; 3],
    _normals: [[f32; 3]; 3],
}

impl Triangle {
    pub fn new(
        positions: [[f32; 3]; 3],
        coordinates: [[f32; 2]; 3],
        normals: [[f32; 3]; 3],
    ) -> Self {
        Self {
            positions,
            _coordinates: coordinates,
            _normals: normals,
        }
    }

    pub fn positions(&self) -> &[[f32; 3]; 3] {
        &self.positions
    }

    pub fn centre(&self) -> [f32; 3] {
        [
            (self.positions[0][0] + self.positions[1][0] + self.positions[2][0]) / 3.0,
            (self.positions[0][1] + self.positions[1][1] + self.positions[2][1]) / 3.0,
            (self.positions[0][2] + self.positions[1][2] + self.positions[2][2]) / 3.0,
        ]
    }

    fn edges(&self) -> [[f32; 3]; 3] {
        [
            [
                self.positions[1][0] - self.positions[0][0],
                self.positions[1][1] - self.positions[0][1],
                self.positions[1][2] - self.positions[0][2],
            ],
            [
                self.positions[2][0] - self.positions[1][0],
                self.positions[2][1] - self.positions[1][1],
                self.positions[2][2] - self.positions[1][2],
            ],
            [
                self.positions[0][0] - self.positions[2][0],
                self.positions[0][1] - self.positions[2][1],
                self.positions[0][2] - self.positions[2][2],
            ],
        ]
    }

    fn cross_product(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn face_normal(&self) -> [f32; 3] {
        let [e1, e2, _e3] = self.edges();
        Self::cross_product(e1, e2)
    }

    fn project(&self, axis: &[f32; 3]) -> (f32, f32) {
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        for &point in &self.positions {
            let val = point[0] * axis[0] + point[1] * axis[1] + point[2] * axis[2];
            min = min.min(val);
            max = max.max(val);
        }
        (min, max)
    }

    pub fn aabb_intersects(&self, aabb: &AABB) -> bool {
        let box_normals = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        let triangle_edges = self.edges();
        let triangle_normal = self.face_normal();
        let aabb_edges = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        let mut axes = box_normals.to_vec();
        axes.push(triangle_normal);
        for &tri_edge in &triangle_edges {
            for &box_edge in &aabb_edges {
                let axis = Self::cross_product(tri_edge, box_edge);
                // Ensure the axis isn't a zero vector.
                if axis != [0.0, 0.0, 0.0] {
                    axes.push(axis);
                }
            }
        }

        for axis in axes.iter() {
            let (t_min, t_max) = self.project(axis);
            let (b_min, b_max) = aabb
                .all_vertices()
                .iter()
                .map(|&vertex| vertex[0] * axis[0] + vertex[1] * axis[1] + vertex[2] * axis[2])
                .fold((f32::INFINITY, f32::NEG_INFINITY), |(min, max), proj| {
                    (min.min(proj), max.max(proj))
                });

            if t_max < b_min || t_min > b_max {
                return false;
            }
        }
        true
    }
}
