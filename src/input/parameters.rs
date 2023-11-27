use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use crate::{
    assets::{Mesh, Resources},
    input::{CameraBuilder, GradientBuilder, InstanceBuilder, MaterialBuilder, SettingsBuilder},
    render::Settings,
    world::{Camera, Scene},
};

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    settings: SettingsBuilder,
    gradients: HashMap<String, GradientBuilder>,
    materials: HashMap<String, MaterialBuilder>,
    meshes: HashMap<String, PathBuf>,
    instances: HashMap<String, InstanceBuilder>,
    cameras: Vec<CameraBuilder>,
}

impl Parameters {
    /// Construct a new instance.
    pub fn new(
        settings: SettingsBuilder,
        gradients: HashMap<String, GradientBuilder>,
        materials: HashMap<String, MaterialBuilder>,
        meshes: HashMap<String, PathBuf>,
        instances: HashMap<String, InstanceBuilder>,
        cameras: Vec<CameraBuilder>,
    ) -> Self {
        let new = Self {
            settings,
            gradients,
            materials,
            meshes,
            instances,
            cameras,
        };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the parameters are all valid.
    pub fn is_valid(&self) -> bool {
        self.settings.is_valid()
            && self
                .gradients
                .iter()
                .all(|(id, gradient_builder)| !id.is_empty() && gradient_builder.is_valid())
            && self
                .materials
                .iter()
                .all(|(id, material_builder)| !id.is_empty() && material_builder.is_valid())
            && self
                .meshes
                .iter()
                .all(|(id, path)| !id.is_empty() && path.exists() && path.is_file())
            && self
                .instances
                .iter()
                .all(|(id, instance)| !id.is_empty() && instance.is_valid())
            && self.cameras.iter().all(|camera| camera.is_valid())
            && self.materials.iter().all(|(_, material)| {
                material
                    .gradient_ids()
                    .iter()
                    .all(|id| self.gradients.contains_key(*id))
            })
            && self.instances.iter().all(|(_, instance)| {
                self.materials.contains_key(instance.material_id())
                    && self.meshes.contains_key(instance.mesh_id())
            })
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

    /// Create the settings.
    pub fn settings(&self) -> Settings {
        self.settings.build()
    }

    /// Load the resources.
    pub fn load_resources(&self, settings: &Settings) -> Resources {
        let mut used_mesh_ids = self
            .instances
            .iter()
            .map(|(_, instance)| instance.mesh_id())
            .collect::<Vec<_>>();
        used_mesh_ids.sort_unstable();
        used_mesh_ids.dedup();
        let mut used_material_ids = self
            .instances
            .iter()
            .map(|(_, instance)| instance.material_id())
            .collect::<Vec<_>>();
        used_material_ids.sort_unstable();
        used_material_ids.dedup();
        let mut used_gradient_ids = used_material_ids
            .iter()
            .flat_map(|material_id| {
                self.materials
                    .get(*material_id)
                    .expect("Unable to find material")
                    .gradient_ids()
            })
            .collect::<Vec<_>>();
        used_gradient_ids.sort_unstable();
        used_gradient_ids.dedup();

        let gradients = used_gradient_ids
            .iter()
            .map(|gradient_id| {
                let gradient_builder = self
                    .gradients
                    .get(*gradient_id)
                    .expect("Unable to find gradient");
                (gradient_id.to_string(), gradient_builder.build())
            })
            .collect::<HashMap<_, _>>();

        let materials = used_material_ids
            .iter()
            .map(|material_id| {
                let material_builder = self
                    .materials
                    .get(*material_id)
                    .expect("Unable to find material");
                (material_id.to_string(), material_builder.build(&gradients))
            })
            .collect::<HashMap<_, _>>();

        let meshes = used_mesh_ids
            .iter()
            .map(|mesh_id| {
                let path = self
                    .meshes
                    .get(*mesh_id)
                    .expect("Unable to find mesh")
                    .clone();
                (
                    mesh_id.to_string(),
                    Mesh::load(&path, settings.mesh_bvh_max_children()),
                )
            })
            .collect::<HashMap<_, _>>();

        Resources::new(meshes, materials)
    }

    /// Create the scene.
    pub fn create_scene<'a>(&self, settings: &Settings, resources: &'a Resources) -> Scene<'a> {
        Scene::new(
            self.instances
                .iter()
                .map(|(_, instance)| instance.build(resources))
                .collect::<Vec<_>>(),
            settings.scene_bvh_max_children(),
        )
    }

    /// Create the cameras.
    pub fn create_cameras(&self) -> Vec<Camera> {
        self.cameras
            .iter()
            .map(|camera_builder| camera_builder.build())
            .collect::<Vec<_>>()
    }
}
