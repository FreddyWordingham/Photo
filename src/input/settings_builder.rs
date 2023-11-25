use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::render::Settings;

/// Input settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsBuilder {
    output_directory: PathBuf,
}

impl SettingsBuilder {
    /// Construct a new instance.
    pub fn new(output_directory: &str) -> Self {
        let new = Self {
            output_directory: output_directory.into(),
        };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the settings parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.output_directory.is_dir()
    }

    /// Build the settings.
    pub fn build(&self) -> Settings {
        Settings::new(&self.output_directory)
    }
}
