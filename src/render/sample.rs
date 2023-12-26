//! Single ray sampling structure.

use core::time::Duration;

use palette::LinSrgba;

/// Scene sample.
pub struct Sample {
    /// Index of the pixel in the image [row, column].
    pixel_index: [usize; 2],
    /// Total colour.
    colour: LinSrgba,
    /// Total time.
    time: Duration,
}
