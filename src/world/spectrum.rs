//! Colour spectra structure.

use enterpolation::{
    linear::{Linear, LinearError},
    Identity, Sorted,
};
use palette::LinSrgba;

/// Sampleable colour map.
pub struct Spectrum {
    /// Colours of the spectrum.
    colours: Linear<Sorted<Vec<f32>>, Vec<LinSrgba>, Identity>,
}

impl Spectrum {
    /// Construct a new instance.
    ///
    /// # Errors
    ///
    /// Returns a [`LinearError`] if the list of colours is empty.
    #[inline]
    pub fn new(colours: Vec<LinSrgba>) -> Result<Self, LinearError> {
        debug_assert!(!colours.is_empty(), "Colours must not be empty!");

        let num_colours = colours.len();
        let delta = 1.0 / (num_colours - 1) as f32;

        Ok(Self {
            colours: Linear::builder()
                .elements(colours)
                .knots(
                    (0..num_colours)
                        .map(|i| i as f32 * delta)
                        .collect::<Vec<_>>(),
                )
                .build()?,
        })
    }
}
