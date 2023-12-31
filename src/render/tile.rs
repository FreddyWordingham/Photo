//! Image tile structure.

use std::{error::Error, path::Path};

use image::{ImageBuffer, Rgba};
use ndarray::Array2;

use crate::{error::SaveError, render::Sample};

/// Image tile.
#[non_exhaustive]
pub struct Tile {
    /// Pixel samples [row, column].
    pub samples: Array2<Sample>,
}

impl Tile {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(tile_index: [usize; 2], resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0, "Resolution must be positive.");
        debug_assert!(resolution[1] > 0, "Resolution must be positive.");

        let offset = [tile_index[0] * resolution[0], tile_index[1] * resolution[1]];
        let samples = Array2::from_shape_fn(resolution, |index| {
            let pixel_index = [offset[0] + index.0, offset[1] + index.1];
            Sample::new(pixel_index)
        });

        Self { samples }
    }

    /// Save the tile to a PNG file.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Tile`] cannot be encoded as a PNG file,
    /// or if the file cannot be saved.
    #[inline]
    pub fn save(&self, file_name: &Path) -> Result<(), Box<dyn Error>> {
        let raw_samples: Vec<_> = self
            .samples
            .iter()
            .flat_map(|sample| {
                let raw: [u8; 4] = sample.colour.into_format().into();
                raw
            })
            .collect();

        let width = self.samples.dim().1.try_into()?;
        let height = self.samples.dim().0.try_into()?;
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_samples)
            .ok_or_else(|| SaveError::new("Failed to create image buffer from raw samples."))?;

        Ok(image.save(&file_name)?)
    }
}
