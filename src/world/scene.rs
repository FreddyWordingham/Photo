use std::collections::HashMap;

use crate::world::{Mesh, Object};

/// Description of the geometry to render.
pub struct Scene {
    /// Triangular meshes.
    pub meshes: HashMap<String, Mesh>,
    /// Objects.
    pub objects: HashMap<String, Object>,
}

impl Scene {
    /// Construct a new Scene object.
    pub fn new(meshes: HashMap<String, Mesh>, objects: HashMap<String, Object>) -> Self {
        Self { meshes, objects }
    }
}
