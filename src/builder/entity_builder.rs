//! Instance builder structure.

use std::collections::HashMap;

use nalgebra::{Similarity3, Translation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

use crate::{
    error::{BuildError, ValidationError},
    geometry::Mesh,
    world::{Entity, Material},
};

/// Builds an [`Entity`] instance.
#[derive(Deserialize, Serialize)]
pub struct EntityBuilder {
    /// Mesh identifier.
    mesh_id: String,
    /// Material identifier.
    material_id: String,
    /// Translation [x, y, z] (meters).
    translation: Option<[f64; 3]>,
    /// Rotation [x, y, z] (degrees).
    rotation: Option<[f64; 3]>,
    /// Scale factor.
    scale: Option<f64>,
}

impl EntityBuilder {
    /// Access the [`Mesh`] identifier.
    #[must_use]
    #[inline]
    pub fn mesh_id(&self) -> &str {
        &self.mesh_id
    }

    /// Access the [`Material`] identifier.
    #[must_use]
    #[inline]
    pub fn material_id(&self) -> &str {
        &self.material_id
    }

    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the [`Mesh`] identifier is empty, or an unknown [`Mesh`] identifier,
    /// or if the [`Material`] identifier is empty, or an unknown [`Material`] identifier,
    /// or if the translation is not finite,
    /// or if the rotation is not finite,
    /// or if the scale is not finite, or not positive.
    #[inline]
    pub fn validate(
        &self,
        material_ids: &[String],
        mesh_ids: &[String],
    ) -> Result<(), ValidationError> {
        if self.mesh_id.is_empty() {
            return Err(ValidationError::new("Mesh identifier may not be empty!"));
        }
        if !mesh_ids.contains(&self.mesh_id) {
            return Err(ValidationError::new(&format!(
                "Unknown mesh identifier: {}!",
                self.mesh_id
            )));
        }

        if self.material_id.is_empty() {
            return Err(ValidationError::new(
                "Material identifier may not be empty!",
            ));
        }
        if !material_ids.contains(&self.material_id) {
            return Err(ValidationError::new(&format!(
                "Unknown material identifier: {}!",
                self.material_id
            )));
        }

        if let Some(translation) = self.translation {
            if !translation.iter().all(|component| component.is_finite()) {
                return Err(ValidationError::new(&format!(
                    "Entity translation must be finite, but the value is [{} {} {}]!",
                    translation[0], translation[1], translation[2]
                )));
            }
        }

        if let Some(rotation) = self.rotation {
            if !rotation.iter().all(|component| component.is_finite()) {
                return Err(ValidationError::new(&format!(
                    "Entity rotation must be finite, but the value is [{} {} {}]!",
                    rotation[0], rotation[1], rotation[2]
                )));
            }
        }

        if let Some(scale) = self.scale {
            if !scale.is_finite() {
                return Err(ValidationError::new(&format!(
                    "Entity scale must be finite, but the value is {scale}!"
                )));
            }
            if scale <= 0.0 {
                return Err(ValidationError::new(&format!(
                    "Entity scale must be positive, but the value is {scale}!"
                )));
            }
        }

        Ok(())
    }

    /// Build an [`Entity`] instance.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if the [`Mesh`] identifier is unknown,
    /// or if the [`Material`] identifier is unknown.
    #[inline]
    pub fn build<'a>(
        &self,
        materials: &'a HashMap<String, Material<'a>>,
        meshes: &'a HashMap<String, Mesh>,
    ) -> Result<Entity<'a>, BuildError> {
        let mesh = meshes
            .get(&self.mesh_id)
            .ok_or_else(|| BuildError::MeshNotFound(self.mesh_id.clone()))?;
        let material = materials
            .get(&self.material_id)
            .ok_or_else(|| BuildError::MaterialNotFound(self.material_id.clone()))?;

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

        Ok(Entity::new(mesh, material, transformation))
    }
}
