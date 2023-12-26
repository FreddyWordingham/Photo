//! Material builder structure.

use serde::{Deserialize, Serialize};

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
