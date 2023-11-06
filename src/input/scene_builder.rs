use serde::{Deserialize, Serialize};

use crate::world::Scene;

/// Runtime scene settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBuilder {}

impl SceneBuilder {
    /// Construct a new instance.
    pub fn new() -> Self {
        let scene_builder = Self {};

        debug_assert!(scene_builder.is_valid());

        scene_builder
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        true
    }

    /// Build a camera from the current settings.
    pub fn build(&self) -> Scene {
        debug_assert!(self.is_valid());

        Scene::new()
    }
}
