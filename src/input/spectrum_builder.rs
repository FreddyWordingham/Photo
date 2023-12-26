//! Spectrum builder structure.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// Builds a [`Spectrum`] instance.
#[derive(Deserialize, Serialize)]
pub struct SpectrumBuilder(Vec<u32>);

impl SpectrumBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the spectrum is empty.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            return Err(ValidationError::new("Spectrum is empty!"));
        }

        Ok(())
    }
}
