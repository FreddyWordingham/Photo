//! Settings builder structure.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// Builds a [`Settings`] instance.
#[derive(Deserialize, Serialize)]
pub struct SettingsBuilder {
    /// Output directory for save files.
    output_directory: PathBuf,
    /// Numerical smoothing length (meters).
    smoothing_length: f64,
    /// Minimum weight of sampling.
    min_weight: f64,
    /// Maximum number of path tracing iterations.
    max_loops: u32,
}

impl SettingsBuilder {
    /// Check if the build parameters are valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the output directory does not exist,
    /// or if the smoothing length is not finite or positive,
    /// or if the minimum weight is not in the range `[0.0, 1.0]`.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.output_directory.is_dir() {
            return Err(ValidationError::new(
                "Output directory must already exist, but it does not!",
            ));
        }

        if !self.smoothing_length.is_finite() {
            return Err(ValidationError::new(&format!(
                "Smoothing length muse be finite, but value is {}!",
                self.smoothing_length
            )));
        }
        if self.smoothing_length <= 0.0 {
            return Err(ValidationError::new(&format!(
                "Smoothing length must be positive, but the value is {}!",
                self.smoothing_length
            )));
        }

        if !(0.0..=1.0).contains(&self.min_weight) {
            return Err(ValidationError::new(&format!(
                "Minimum weight must be in the range [0.0, 1.0], but the value is {}!",
                self.min_weight
            )));
        }

        Ok(())
    }
}
