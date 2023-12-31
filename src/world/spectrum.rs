//! Colour spectra structure.

use enterpolation::{linear::Linear, linear::LinearError, Generator, Identity, Sorted};
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

    /// Sample the gradient linearly for a colour at a given point in the range [0, 1].
    #[must_use]
    #[inline]
    #[allow(clippy::min_ident_chars)]
    pub fn sample(&self, t: f32) -> LinSrgba {
        debug_assert!(
            (0.0..=1.0).contains(&t),
            "Sample point must be in the range [0, 1]!"
        );

        // let t = round_to_nearest(t, 0.33);

        self.colours.sample([t]).collect::<Vec<_>>()[0]
    }

    /// Sample the gradient linearly for a colour at a given point in the range [0, 1], in steps.
    #[must_use]
    #[inline]
    #[allow(clippy::min_ident_chars)]
    pub fn sample_in_steps(&self, t: f32, step: f32) -> LinSrgba {
        debug_assert!(
            (0.0..=1.0).contains(&t),
            "Sample point must be in the range [0, 1]!"
        );
        debug_assert!(
            (0.0..=1.0).contains(&step),
            "Sample step must be in the range [0, 1]!"
        );

        self.colours
            .sample([(t / step).round() * step])
            .collect::<Vec<_>>()[0]
    }
}
