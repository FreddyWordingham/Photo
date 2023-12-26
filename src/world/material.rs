//! Surface material enumeration.

use crate::world::Spectrum;

/// Surface materials.
#[non_exhaustive]
pub enum Material<'a> {
    /// Opaque material.
    Diffuse {
        /// Spectrum colour of the material.
        spectrum: &'a Spectrum,
    },
    /// Partially reflective material.
    Reflective {
        /// Spectrum colour of the material.
        spectrum: &'a Spectrum,
        /// Fraction of light absorbed by the material.
        absorption: f64,
    },
    /// Partially reflective, partially transmissive material.
    Refractive {
        /// Spectrum colour of the material.
        spectrum: &'a Spectrum,
        /// Fraction of light absorbed by the material.
        absorption: f64,
        /// Refractive index of the material.
        refractive_index: f64,
    },
}
