//! Parameters builder structure.

extern crate alloc;

use alloc::borrow::ToOwned;
use std::{
    collections::HashMap,
    error::Error,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use enterpolation::linear::LinearError;
use serde::{Deserialize, Serialize};

use crate::{
    builder::{
        CameraBuilder, EntityBuilder, LightBuilder, MaterialBuilder, SettingsBuilder,
        SpectrumBuilder,
    },
    error::{BuildError, ValidationError},
    geometry::Mesh,
    render::Settings,
    world::{Camera, Entity, Light, Material, Spectrum},
};

/// Input parameters object.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub struct Parameters {
    /// Settings builder.
    pub settings: SettingsBuilder,
    /// Spectrum builders.
    pub spectra: HashMap<String, SpectrumBuilder>,
    /// Material builders.
    pub materials: HashMap<String, MaterialBuilder>,
    /// Mesh paths.
    pub meshes: HashMap<String, PathBuf>,
    /// Entity builders.
    pub entities: Vec<EntityBuilder>,
    /// Light builders.
    pub lights: Vec<LightBuilder>,
    /// Camera builder.
    pub cameras: HashMap<String, CameraBuilder>,
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
        debug_assert!(path.is_file(), "Path must be a file!");

        let file_string = read_to_string(path)?;
        Ok(serde_yaml::from_str(&file_string)?)
    }

    /// Get the list of unique spectrum identifiers that are used by materials which are used by the entities.
    #[must_use]
    #[inline]
    pub fn used_spectrum_ids(&self) -> Vec<String> {
        let material_ids = self.used_material_ids();

        let mut spectrum_ids = material_ids
            .iter()
            .flat_map(|material_id| {
                self.materials[material_id]
                    .spectrum_ids()
                    .into_iter()
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        spectrum_ids.sort();
        spectrum_ids.dedup();

        spectrum_ids
    }

    /// Get the list of unique material identifiers that are used by the entities.
    #[must_use]
    #[inline]
    pub fn used_material_ids(&self) -> Vec<String> {
        let mut material_ids = self
            .entities
            .iter()
            .map(|entity| entity.material_id().to_owned())
            .collect::<Vec<_>>();

        material_ids.sort();
        material_ids.dedup();

        material_ids
    }

    /// Get the list of unique mesh identifiers that are used by the entities.
    #[must_use]
    #[inline]
    pub fn used_mesh_ids(&self) -> Vec<String> {
        let mut mesh_ids = self
            .entities
            .iter()
            .map(|entity| entity.mesh_id().to_owned())
            .collect::<Vec<_>>();

        mesh_ids.sort();
        mesh_ids.dedup();

        mesh_ids
    }

    /// Check if all the parameters are valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if any of the parameters are invalid,
    /// or if any of the [`Mesh`] paths are invalid,
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
        let spectrum_ids = self
            .spectra
            .keys()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        self.materials.iter().try_for_each(|(id, material)| {
            if id.is_empty() {
                return Err(ValidationError::new("Material identifier is empty!"));
            }
            MaterialBuilder::validate(material, &spectrum_ids)
        })?;

        self.meshes.iter().try_for_each(|(id, path)| {
            if id.is_empty() {
                return Err(ValidationError::new("Mesh identifier is empty!"));
            }
            if !path.is_file() {
                return Err(ValidationError::new(&format!(
                    "Mesh path does not exist: {}!",
                    path.display()
                )));
            }
            Ok(())
        })?;

        let material_ids = self
            .materials
            .keys()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let mesh_ids = self
            .meshes
            .keys()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        self.entities
            .iter()
            .try_for_each(|entity| EntityBuilder::validate(entity, &material_ids, &mesh_ids))?;

        self.lights.iter().try_for_each(LightBuilder::validate)?;

        self.cameras.iter().try_for_each(|(id, camera)| {
            if id.is_empty() {
                return Err(ValidationError::new("Camera identifier is empty!"));
            }
            CameraBuilder::validate(camera)
        })?;

        Ok(())
    }

    /// Build the [`Settings`] instance.
    #[must_use]
    #[inline]
    pub fn build_settings(&self) -> Settings {
        self.settings.build()
    }

    /// Build the collection of [`Spectra`] instances.
    ///
    /// # Errors
    ///
    /// Returns a [`LinearError`] if a [`Spectrum`] cannot be built.
    #[inline]
    pub fn build_spectra(&self) -> Result<HashMap<String, Spectrum>, LinearError> {
        self.used_spectrum_ids()
            .iter()
            .map(|id| {
                let builder = &self.spectra[id];
                let spectrum = builder.build()?;
                Ok((id.clone(), spectrum))
            })
            .collect()
    }

    /// Build the collection of [`Material`] instances.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if a [`Material`] cannot be built.
    #[inline]
    pub fn build_materials<'a>(
        &self,
        spectra: &'a HashMap<String, Spectrum>,
    ) -> Result<HashMap<String, Material<'a>>, BuildError> {
        self.used_material_ids()
            .iter()
            .map(|id| {
                let builder = &self.materials[id];
                let material = builder.build(spectra)?;
                Ok((id.clone(), material))
            })
            .collect()
    }

    /// Build the collection of [`Mesh`] instances.
    ///
    /// # Errors
    ///
    /// Returns a [`Box<dyn Error>`] if a [`Mesh`] cannot be built.
    #[inline]
    pub fn build_meshes(
        &self,
        bvh_max_children: usize,
        bvh_max_depth: usize,
    ) -> Result<HashMap<String, Mesh>, Box<dyn Error>> {
        debug_assert!(
            bvh_max_children >= 2,
            "Mesh BVH max children must be greater than 2!"
        );
        debug_assert!(bvh_max_depth > 0, "Mesh BVH max depth must be positive!");

        self.used_mesh_ids()
            .iter()
            .map(|id| {
                let path = &self.meshes[id];
                let mesh = Mesh::load(path, bvh_max_children, bvh_max_depth)?;
                Ok((id.clone(), mesh))
            })
            .collect()
    }

    /// Build the collection of [`Entity`] instances.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] if an [`Entity`] cannot be built.
    #[inline]
    pub fn build_entities<'a>(
        &self,
        materials: &'a HashMap<String, Material<'a>>,
        meshes: &'a HashMap<String, Mesh>,
    ) -> Result<Vec<Entity<'a>>, BuildError> {
        self.entities
            .iter()
            .map(|builder| builder.build(materials, meshes))
            .collect()
    }

    /// Build the collection of [`Light`] instances.
    #[must_use]
    #[inline]
    pub fn build_lights(&self) -> Vec<Light> {
        self.lights.iter().map(LightBuilder::build).collect()
    }

    /// Build the collection of [`Camera`] instances.
    #[must_use]
    #[inline]
    pub fn build_cameras(&self) -> HashMap<String, Camera> {
        self.cameras
            .iter()
            .map(|(key, builder)| {
                let camera = builder.build();
                (key.clone(), camera)
            })
            .collect()
    }
}
