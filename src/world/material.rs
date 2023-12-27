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

impl<'a> Material<'a> {
    /// Construct a new Diffuse [`Material`] instance.
    #[must_use]
    #[inline]
    pub const fn new_diffuse(spectrum: &'a Spectrum) -> Self {
        Self::Diffuse { spectrum }
    }

    /// Construct a new Reflective [`Material`] instance.
    #[must_use]
    #[inline]
    pub const fn new_reflective(spectrum: &'a Spectrum, absorption: f64) -> Self {
        Self::Reflective {
            spectrum,
            absorption,
        }
    }

    /// Construct a new Refractive [`Material`] instance.
    #[must_use]
    #[inline]
    pub const fn new_refractive(
        spectrum: &'a Spectrum,
        absorption: f64,
        refractive_index: f64,
    ) -> Self {
        Self::Refractive {
            spectrum,
            absorption,
            refractive_index,
        }
    }
}
