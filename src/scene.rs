use crate::Mesh;

pub struct Scene {
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn load_mesh(&mut self, path: &str) {
        self.meshes.push(Mesh::load(path));
    }

    pub fn positions_data(&self) -> Vec<f32> {
        self.meshes[0].positions_data()
    }

    pub fn position_indices_data(&self) -> Vec<u32> {
        self.meshes[0].position_indices_data()
    }

    pub fn normals_data(&self) -> Vec<f32> {
        self.meshes[0].normals_data()
    }

    pub fn normal_indices_data(&self) -> Vec<u32> {
        self.meshes[0].normal_indices_data()
    }
}
