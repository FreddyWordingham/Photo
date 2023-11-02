use crate::geometry::{Mesh, AABB};

pub struct Scene {
    aabb: AABB,
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        debug_assert!(!meshes.is_empty());
        debug_assert!(meshes.iter().all(|mesh| mesh.is_valid()));

        let mut mins = [std::f32::MAX; 3];
        let mut maxs = [-std::f32::MAX; 3];
        for mesh in &meshes {
            let mesh_mins = mesh.aabb().mins();
            let mesh_maxs = mesh.aabb().maxs();

            for (n, min) in mins.iter_mut().enumerate() {
                if mesh_mins[n] < *min {
                    *min = mesh_mins[n];
                }
            }

            for (n, max) in maxs.iter_mut().enumerate() {
                if mesh_maxs[n] > *max {
                    *max = mesh_maxs[n];
                }
            }
        }

        Self {
            aabb: AABB::new(mins, maxs),
            meshes,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }

    pub fn is_valid(&self) -> bool {
        self.aabb.is_valid() && self.meshes.iter().all(|mesh| mesh.is_valid())
    }

    pub fn positions_buffer(&self) -> Vec<f32> {
        let mut buffer = Vec::new();

        for (n, mesh) in self.meshes.iter().enumerate() {
            let mut positions = mesh.positions_buffer(n as f32);
            buffer.append(&mut positions);
        }

        buffer
    }

    pub fn position_indices_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::new();

        fn max_of_buffer(buffer: &[u32]) -> i32 {
            let mut max: i32 = -1;
            for (n, i) in buffer.iter().enumerate() {
                if n % 4 == 0 {
                    continue;
                }

                if *i as i32 > max {
                    max = *i as i32;
                }
            }
            max
        }

        for (n, mesh) in self.meshes.iter().enumerate() {
            let mut offset = max_of_buffer(&buffer) + 1;
            if offset < 0 {
                offset = 0;
            }
            let mut indices = mesh.position_indices_buffer(n as u32, offset as u32);

            buffer.append(&mut indices);
        }

        buffer
    }

    pub fn normals_buffer(&self) -> Vec<f32> {
        let mut buffer = Vec::new();

        for (n, mesh) in self.meshes.iter().enumerate() {
            let mut indices = mesh.normals_buffer(n as f32);
            buffer.append(&mut indices);
        }

        buffer
    }

    pub fn normal_indices_buffer(&self) -> Vec<u32> {
        let mut buffer = Vec::new();

        fn max_of_buffer(buffer: &[u32]) -> i32 {
            let mut max: i32 = -1;
            for (n, i) in buffer.iter().enumerate() {
                if n % 4 == 0 {
                    continue;
                }

                if *i as i32 > max {
                    max = *i as i32;
                }
            }
            max
        }

        for (n, mesh) in self.meshes.iter().enumerate() {
            let mut offset = max_of_buffer(&buffer) + 1;
            if offset < 0 {
                offset = 0;
            }
            let mut indices = mesh.normal_indices_buffer(n as u32, offset as u32);
            buffer.append(&mut indices);
        }

        buffer
    }
}
