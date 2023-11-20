use crate::geometry::Mesh;

pub struct Resources {
    meshes: Vec<Mesh>,
}

impl Resources {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes
    }
}
