//! Image tile structure.

use ndarray::Array2;

use crate::render::Sample;

/// Image tile.
pub struct Tile {
    /// Pixel samples [row, column].
    samples: Array2<Sample>,
}
