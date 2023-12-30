//! Material builder structure.

use serde::{Deserialize, Serialize};

use crate::{engine, error::ValidationError, geometry::Ray, render::Sample, world::Scene};

/// Parametrises an [`engine`] function.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum EngineBuilder {
    /// Stencil.
    Stencil {},
    /// Outlined.
    Outlined { width: f64 },
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
            Self::Stencil {} => Ok(()),
            Self::Outlined { width } => {
                if !width.is_finite() {
                    return Err(ValidationError::new(&format!(
                        "Outline width must be finite, but the value is {}!",
                        width
                    )));
                }
                if *width < 0.0 {
                    return Err(ValidationError::new(&format!(
                        "Outline width must be positive, but the value is {}!",
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
    pub fn build(&self) -> fn(&Scene, [usize; 2], &Ray) -> Sample {
        match self {
            Self::Stencil {} => engine::stencil,
            Self::Outlined { width } => engine::outlined(*width),
        }
    }
}
