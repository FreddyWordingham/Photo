use serde::{Deserialize, Serialize};

use crate::world::Scene;

/// Runtime scene settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBuilder {}

impl SceneBuilder {
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
