//! Material builder structure.

use serde::{Deserialize, Serialize};

use crate::{engine, engine::Engine, error::ValidationError};

/// Parametrises an [`engine`] function.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum EngineBuilder {
    /// Stencil.
    Stencil,
    /// Outlined.
    Outlined(f64),
}

impl EngineBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the [`engine`] configuration is invalid.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Self::Stencil => Ok(()),
            Self::Outlined(width) => {
                if !width.is_finite() {
                    return Err(ValidationError::new(&format!(
                        "Engine-Outline: width parameter must be finite, but the value is {}!",
                        width
                    )));
                }
                if *width < 0.0 {
                    return Err(ValidationError::new(&format!(
                        "Engine-Outline: width parameter must be positive, but the value is {}!",
                        width
                    )));
                }

                Ok(())
            }
        }
    }

    /// Build a [`engine`] function handle.
    #[must_use]
    #[inline]
    pub fn build(&self) -> Engine {
        match self {
            Self::Stencil => {
                Box::new(|scene, pixel_index, ray| engine::stencil(scene, pixel_index, ray))
            }
            Self::Outlined(width) => {
                println!("Outline width: {}", width);
                Box::new(|scene, pixel_index, ray| engine::stencil(scene, pixel_index, ray))
            }
        }
    }
}
