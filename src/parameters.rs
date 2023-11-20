use serde::{Deserialize, Serialize};

use crate::{Camera, Resources};

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {}

impl Parameters {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Load from a file.

    /// Create the cameras.
    pub fn create_cameras(&self) -> Vec<Camera> {
        let cameras = vec![];
        cameras
    }

    /// Load the resources.
    pub fn create_resources(&self) -> Resources {
        let meshes = vec![];
        Resources::new(meshes)
    }
}
