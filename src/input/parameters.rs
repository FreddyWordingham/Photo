//! Parameters builder structure.

use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    error::ValidationError,
    input::{CameraBuilder, EntityBuilder, MaterialBuilder, SettingsBuilder, SpectrumBuilder},
};

/// Input parameters object.
#[derive(Deserialize, Serialize)]
pub struct Parameters {
    /// Settings builder.
    settings: SettingsBuilder,
    /// Spectrum builders.
    spectra: HashMap<String, SpectrumBuilder>,
    /// Material builders.
    materials: HashMap<String, MaterialBuilder>,
    /// Entity builders.
    entities: Vec<EntityBuilder>,
    /// Camera builder.
    cameras: HashMap<String, CameraBuilder>,
}

impl Parameters {
    /// Load a Parameters object from a file.
    ///
    /// # Errors
    ///
    /// Returns a [`Box<dyn Error>`] if the file cannot be read,
    /// or if the file cannot be deserialized into a [`Parameters`] object.
    #[inline]
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file_string = read_to_string(path)?;
        Ok(serde_yaml::from_str(&file_string)?)
    }

    /// Check if all the parameters are valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if any of the parameters are invalid,
    /// or if any of the identifiers are empty.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        self.settings.validate()?;

        self.spectra.iter().try_for_each(|(id, spectrum)| {
            if id.is_empty() {
                return Err(ValidationError::new("Spectrum identifier is empty!"));
            }
            SpectrumBuilder::validate(spectrum)
        })?;
        let spectra_ids = self.spectra.keys().cloned().collect::<Vec<_>>();

        self.materials.iter().try_for_each(|(id, material)| {
            if id.is_empty() {
                return Err(ValidationError::new("Material identifier is empty!"));
            }
            MaterialBuilder::validate(material, &spectra_ids)
        })?;
        let material_ids = self.materials.keys().cloned().collect::<Vec<_>>();

        self.entities
            .iter()
            .try_for_each(|entity| EntityBuilder::validate(entity, &spectra_ids, &material_ids))?;

        self.cameras.iter().try_for_each(|(id, camera)| {
            if id.is_empty() {
                return Err(ValidationError::new("Camera identifier is empty!"));
            }
            CameraBuilder::validate(camera)
        })?;

        Ok(())
    }
}
