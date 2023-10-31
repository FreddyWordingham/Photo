use crate::Mesh;

pub struct Scene {
    meshes: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Self {
        Self { meshes: Vec::new() }
    }

    pub fn is_valid(&self) -> bool {
        self.meshes.iter().all(|mesh| mesh.is_valid())
    }

    pub fn load_mesh(&mut self, path: &str) {
        self.meshes.push(Mesh::load(path));
    }

    pub fn positions_data(&self) -> Vec<f32> {
        let tag = 0.0;
        self.meshes[0].positions_data(tag)
    }

    pub fn position_indices_data(&self) -> Vec<u32> {
        let tag = 0;
        let offset = 0;
        self.meshes[0].position_indices_data(tag, offset)
    }

    pub fn normals_data(&self) -> Vec<f32> {
        let tag = 0.0;
        self.meshes[0].normals_data(tag)
    }

    pub fn normal_indices_data(&self) -> Vec<u32> {
        let tag = 0;
        let offset = 0;
        self.meshes[0].normal_indices_data(tag, offset)
    }
}
