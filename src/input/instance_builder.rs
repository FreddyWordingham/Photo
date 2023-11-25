use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceBuilder {
    mesh_id: String,
    material_id: String,
    translation: Option<[f64; 3]>,
    rotation: Option<[f64; 3]>,
    scale: Option<f64>,
}

impl InstanceBuilder {
    pub fn new(mesh_id: String, material_id: String) -> Self {
        let new = Self {
            mesh_id,
            material_id,
            translation: None,
            rotation: None,
            scale: None,
        };

        debug_assert!(new.is_valid());

        new
    }

    pub fn mesh_id(&self) -> &str {
        &self.mesh_id
    }

    pub fn material_id(&self) -> &str {
        &self.material_id
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

    /// Check if the instance parameters are valid.
    pub fn is_valid(&self) -> bool {
        !self.mesh_id.is_empty()
            && !self.material_id.is_empty()
            && (self.translation.is_none()
                || self.translation.unwrap().iter().all(|x| x.is_finite()))
            && (self.rotation.is_none() || self.rotation.unwrap().iter().all(|x| x.is_finite()))
            && (self.scale.is_none() || self.scale.unwrap() > 0.0)
    }
}
