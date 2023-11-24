use serde::{Deserialize, Serialize};

use crate::render::Settings;

/// Input settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsBuilder {}

impl SettingsBuilder {
    /// Construct a new instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Validate the settings.
    pub fn is_valid(&self) -> bool {
        true
    }

    /// Build the settings.
    pub fn build(&self) -> Settings {
        Settings::new()
    }
}
