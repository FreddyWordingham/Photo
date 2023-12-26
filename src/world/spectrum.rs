//! Colour spectra structure.

use enterpolation::{linear::Linear, Identity, Sorted};
use palette::LinSrgba;

/// Sampleable colour map.
pub struct Spectrum {
    /// Colours of the spectrum.
    colours: Linear<Sorted<Vec<f32>>, Vec<LinSrgba>, Identity>,
}
