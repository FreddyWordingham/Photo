use nalgebra::Point3;
use std::path::{Path, PathBuf};

/// Input settings.
pub struct Settings {
    output_directory: PathBuf,
    sun_position: Point3<f64>,
}

impl Settings {
    pub fn new(output_directory: &Path, sun_position: Point3<f64>) -> Self {
        Self {
            output_directory: output_directory.to_path_buf(),
            sun_position,
        }
    }

    pub fn output_directory(&self) -> &Path {
        &self.output_directory
    }

    pub fn sun_position(&self) -> &Point3<f64> {
        &self.sun_position
    }
}
