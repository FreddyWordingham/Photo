use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::render::Settings;

/// Input settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsBuilder {
    output_directory: PathBuf,
    sun_position: [f64; 3],
}

impl SettingsBuilder {
    /// Construct a new instance.
    pub fn new(output_directory: &str, sun_position: [f64; 3]) -> Self {
        let new = Self {
            output_directory: output_directory.into(),
            sun_position,
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
        Settings::new(&self.output_directory, self.sun_position.into())
    }
}
