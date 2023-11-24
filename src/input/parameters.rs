use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::Path,
};

use crate::{
    assets::Resources,
    input::{CameraBuilder, InstanceBuilder, SettingsBuilder},
    world::{Camera, Scene},
};

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    settings: SettingsBuilder,
    meshes: HashMap<String, String>,
    instances: HashMap<String, InstanceBuilder>,
    cameras: Vec<CameraBuilder>,
}

impl Parameters {
    /// Construct a new instance.
    pub fn new(
        settings: SettingsBuilder,
        meshes: HashMap<String, String>,
        instances: HashMap<String, InstanceBuilder>,
        cameras: Vec<CameraBuilder>,
    ) -> Self {
        debug_assert!(settings.is_valid());

        Self {
            settings,
            meshes,
            instances,
            cameras,
        }
    }

    /// Load a Parameters object from a file.
    pub fn load(path: &Path) -> Self {
        let file_string = read_to_string(path).expect("Unable to read settings file");
        serde_yaml::from_str(&file_string).expect("Unable to parse settings file")
    }

    /// Get this as YAML.
    pub fn as_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("Unable to serialise Parameters object to YAML string")
    }

    /// Save the settings to the given file.
    pub fn save(&self, path: &Path) {
        write(path, self.as_yaml()).expect("Unable to write Parameters object to file");
    }

    /// Load the resources.
    pub fn load_resources(&self) -> Resources {
        todo!()
    }

    /// Create the scene.
    pub fn create_scene<'a>(&self, _resources: &'a Resources) -> Scene<'a> {
        todo!()
    }

    /// Create the cameras.
    pub fn create_cameras(&self) -> Vec<Camera> {
        todo!()
    }
}
