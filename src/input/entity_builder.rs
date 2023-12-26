//! Instance builder structure.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

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
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the mesh identifier is empty, or an unknown mesh identifier,
    /// or if the material identifier is empty, or an unknown material identifier,
    /// or if the translation is not finite,
    /// or if the rotation is not finite,
    /// or if the scale is not finite, or not positive.
    #[inline]
    pub fn validate(
        &self,
        mesh_ids: &[String],
        material_ids: &[String],
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
}
