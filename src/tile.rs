use ndarray::Array2;

use crate::Sample;

/// A tile is a rectangular region of the total image.
pub struct Tile {
    /// Index of the tile in the image.
    pub tile_index: [usize; 2],
    /// Data
    pub data: Array2<Sample>,
}

impl Tile {
    /// Construct a new Tile object.
    pub fn new(tile_index: [usize; 2], resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        Self {
            tile_index,
            data: Array2::<Sample>::from_shape_fn(resolution, |index| Sample::new(index)),
        }
    }
}
