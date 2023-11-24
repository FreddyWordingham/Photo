use std::collections::HashMap;

use crate::assets::{Gradient, Material, Mesh};

pub struct Resources {
    meshes: HashMap<String, Mesh>,
    colours: HashMap<String, Gradient>,
    materials: HashMap<String, Material>,
}

impl Resources {
    pub fn new(
        meshes: HashMap<String, Mesh>,
        colours: HashMap<String, Gradient>,
        materials: HashMap<String, Material>,
    ) -> Self {
        Self {
            meshes,
            colours,
            materials,
        }
    }

    pub fn meshes(&self) -> &HashMap<String, Mesh> {
        &self.meshes
    }

    pub fn colours(&self) -> &HashMap<String, Gradient> {
        &self.colours
    }

    pub fn materials(&self) -> &HashMap<String, Material> {
        &self.materials
    }
}
