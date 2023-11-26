use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Input settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    output_directory: PathBuf,
}

impl Settings {
    pub fn new(output_directory: &Path) -> Self {
        Self {
            output_directory: output_directory.to_path_buf(),
        }
    }

    pub fn output_directory(&self) -> &Path {
        &self.output_directory
    }
}
