use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    input::ObjectBuilder,
    world::{Mesh, Scene},
};

/// Runtime scene settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBuilder {
    /// Mesh file paths.
    pub meshes: HashMap<String, PathBuf>,
    /// Scene objects.
    pub objects: HashMap<String, ObjectBuilder>,
}

impl SceneBuilder {
    /// Construct a new instance.
    pub fn new(meshes: HashMap<String, PathBuf>, objects: HashMap<String, ObjectBuilder>) -> Self {
        let scene_builder = Self { meshes, objects };

        debug_assert!(scene_builder.is_valid());

        scene_builder
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        for (mesh_id, file_path) in &self.meshes {
            if !file_path.exists() {
                println!(
                    "INVALID! Mesh file does not exist: {} -> {}",
                    mesh_id,
                    file_path.display()
                );
                return false;
            }
        }

        for (object_id, object) in &self.objects {
            if !self.meshes.contains_key(&object.mesh_id) {
                println!(
                    "INVALID! Object {} requires mesh_id: {}",
                    object_id, object.mesh_id
                );
                return false;
            }
        }

        true
    }

    /// Build a scene from the current settings.
    pub fn build(&self) -> Scene {
        debug_assert!(self.is_valid());

        let meshes: HashMap<_, _> = self
            .meshes
            .iter()
            .map(|(mesh_id, file_path)| (mesh_id.clone(), Mesh::load(file_path)))
            .collect();

        let objects: HashMap<_, _> = self
            .objects
            .iter()
            .map(|(object_id, object_builder)| {
                (
                    object_id.clone(),
                    object_builder.build(&meshes[&object_builder.mesh_id]),
                )
            })
            .collect();

        Scene::new(meshes, objects)
    }
}
