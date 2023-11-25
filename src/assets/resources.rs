use std::collections::HashMap;

use crate::assets::{Material, Mesh};

pub struct Resources {
    meshes: HashMap<String, Mesh>,
    materials: HashMap<String, Material>,
}

impl Resources {
    pub fn new(meshes: HashMap<String, Mesh>, materials: HashMap<String, Material>) -> Self {
        Self { meshes, materials }
    }

    pub fn meshes(&self) -> &HashMap<String, Mesh> {
        &self.meshes
    }

    pub fn materials(&self) -> &HashMap<String, Material> {
        &self.materials
    }
}
