//! Material builder structure.

use serde::{Deserialize, Serialize};

use crate::error::ValidationError;

/// Builds a [`Material`] instance.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum MaterialBuilder {
    /// Opaque material.
    Diffuse {
        /// Spectrum colour of the material.
        spectrum: String,
    },
    /// Partially reflective material.
    Reflective {
        /// Spectrum colour of the material.
        spectrum: String,
        /// Fraction of light absorbed by the material.
        absorption: f64,
    },
    /// Partially reflective, partially transmissive material.
    Refractive {
        /// Spectrum colour of the material.
        spectrum: String,
        /// Fraction of light absorbed by the material.
        absorption: f64,
        /// Refractive index of the material.
        refractive_index: f64,
    },
}

impl MaterialBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the spectrum identifier is invalid,
    /// or if absorption value is invalid,
    /// or if the refractive index value is invalid.
    #[inline]
    pub fn validate(&self, spectra_ids: &[String]) -> Result<(), ValidationError> {
        let (spectrum, absorption, refractive_index) = match self {
            Self::Diffuse { spectrum } => (spectrum, None, None),
            Self::Reflective {
                spectrum,
                absorption,
            } => (spectrum, Some(absorption), None),
            Self::Refractive {
                spectrum,
                absorption,
                refractive_index,
            } => (spectrum, Some(absorption), Some(refractive_index)),
        };

        Self::validate_spectrum(spectrum, spectra_ids)?;
        if let Some(absorption) = absorption {
            Self::validate_absorption(*absorption)?;
        }
        if let Some(refractive_index) = refractive_index {
            Self::validate_refractive_index(*refractive_index)?;
        }

        Ok(())
    }

    /// Check if the spectrum is a valid identifier.
    /// It must be a non-empty string and it must exist in the list of known spectra identifiers.
    fn validate_spectrum(spectrum: &String, spectra_ids: &[String]) -> Result<(), ValidationError> {
        if spectrum.is_empty() {
            return Err(ValidationError::new("Spectrum identifier is empty!"));
        }
        if !spectra_ids.contains(spectrum) {
            return Err(ValidationError::new(&format!(
                "Unknown spectrum identifier {}!",
                spectrum
            )));
        }
        Ok(())
    }

    /// Check if the absorption is a valid value, i.e. in the range [0.0, 1.0].
    fn validate_absorption(absorption: f64) -> Result<(), ValidationError> {
        if !(0.0..=1.0).contains(&absorption) {
            return Err(ValidationError::new(&format!(
                "Absorption mut be in the range [0.0, 1.0], but it is {}!",
                absorption
            )));
        }
        Ok(())
    }

    /// Check if the refractive index is a valid value, i.e. greater than or equal to 1.0.
    fn validate_refractive_index(refractive_index: f64) -> Result<(), ValidationError> {
        if !refractive_index.is_finite() {
            return Err(ValidationError::new(&format!(
                "Refractive index must be finite, but the value is {}!",
                refractive_index
            )));
        }

        if refractive_index <= 1.0 {
            return Err(ValidationError::new(&format!(
                "Refractive index must be greater than, or equal to 1.0, but the value is {}!",
                refractive_index
            )));
        }

        Ok(())
    }
}
