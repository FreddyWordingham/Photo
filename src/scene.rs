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

    pub fn positions_buffer(&self) -> Vec<f32> {
        let tag = 0.0;
        self.meshes[0].positions_buffer(tag)
    }

    pub fn position_indices_buffer(&self) -> Vec<u32> {
        let tag = 0;
        let offset = 0;
        self.meshes[0].position_indices_buffer(tag, offset)
    }

    pub fn normals_buffer(&self) -> Vec<f32> {
        let tag = 0.0;
        self.meshes[0].normals_buffer(tag)
    }

    pub fn normal_indices_buffer(&self) -> Vec<u32> {
        let tag = 0;
        let offset = 0;
        self.meshes[0].normal_indices_buffer(tag, offset)
    }
}
