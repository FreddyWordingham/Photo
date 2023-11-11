use std::collections::HashMap;

use crate::world::Mesh;

/// Description of the geometry to render.
pub struct Scene {
    /// Triangular meshes.
    pub meshes: HashMap<String, Mesh>,
}

impl Scene {
    /// Construct a new Scene object.
    pub fn new(meshes: HashMap<String, Mesh>) -> Self {
        Self { meshes }
    }
}
