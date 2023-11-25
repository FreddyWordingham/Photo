use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

use crate::{assets::Resources, world::Instance};

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

    pub fn build<'a>(&self, resources: &'a Resources) -> Instance<'a> {
        let mesh = resources.meshes().get(&self.mesh_id).unwrap();
        let material = resources.materials().get(&self.material_id).unwrap();

        let translation = self.translation.unwrap_or([0.0; 3]);
        let rotation = self.rotation.unwrap_or([0.0; 3]);
        let scale = self.scale.unwrap_or(1.0);
        let transformation = Similarity3::from_parts(
            Translation3::from(Vector3::new(translation[0], translation[1], translation[2])),
            UnitQuaternion::from_euler_angles(
                rotation[0].to_radians(),
                rotation[1].to_radians(),
                rotation[2].to_radians(),
            ),
            scale,
        );

        Instance::new(mesh, material, transformation)
    }
}
