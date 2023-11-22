use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceBuilder {
    mesh_id: String,
    translation: Option<[f64; 3]>,
    rotation: Option<[f64; 3]>,
    scale: Option<f64>,
}

impl InstanceBuilder {
    pub fn new(mesh_id: String) -> Self {
        Self {
            mesh_id,
            translation: None,
            rotation: None,
            scale: None,
        }
    }

    pub fn with_translation(mut self, translation: [f64; 3]) -> Self {
        self.translation = Some(translation);
        self
    }

    pub fn with_rotation(mut self, rotation: [f64; 3]) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }
}
