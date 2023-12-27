//! Spectrum builder structure.

use enterpolation::linear::LinearError;
use serde::{Deserialize, Serialize};

use crate::{error::ValidationError, utility::colour::from_u32, world::Spectrum};

/// Builds a [`Spectrum`] instance.
#[derive(Deserialize, Serialize)]
pub struct SpectrumBuilder(Vec<u32>);

impl SpectrumBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the spectrum is empty,
    /// or if any of the colours are not a valid 32-bit RGBA colour.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            return Err(ValidationError::new(
                "Spectrum colours vector must not be empty!",
            ));
        }

        Ok(())
    }

    /// Build a [`Spectrum`] instance.
    ///
    /// # Errors
    ///
    /// Returns a [`LinearError`] if the list of colours is empty.
    #[inline]
    pub fn build(&self) -> Result<Spectrum, LinearError> {
        let colours = self
            .0
            .iter()
            .map(|colour| from_u32(*colour))
            .collect::<Vec<_>>();
        Spectrum::new(colours)
    }
}
