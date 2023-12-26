//! Surface material enumeration.

use crate::world::Spectrum;

/// Surface materials.
pub enum Material<'a> {
    /// Opaque material.
    Diffuse {
        /// Colour of the material.
        colour: &'a Spectrum,
    },
    /// Partially reflective material.
    Reflective {
        /// Colour of the material.
        colour: &'a Spectrum,
        /// Fraction of light absorbed by the material.
        absorption: f64,
    },
    /// Partially reflective, partially transmissive material.
    Refractive {
        /// Colour of the material.
        colour: &'a Spectrum,
        /// Fraction of light absorbed by the material.
        absorption: f64,
        /// Refractive index of the material.
        refractive_index: f64,
    },
}
