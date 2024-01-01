//! Image tile structure.

use core::num::TryFromIntError;
use std::{error::Error, path::Path};

use image::{ImageBuffer, Rgba};
use ndarray::Array2;

use crate::{error::SaveError, render::Sample};

/// Image tile.
#[non_exhaustive]
pub struct Tile {
    /// Tile index [row, column].
    pub tile_index: [usize; 2],
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

        Self {
            tile_index,
            samples,
        }
    }

    /// Save the [`Tile`] to PNG files.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Tile`] cannot be encoded as a PNG file,
    /// or if the file cannot be saved.
    #[inline]
    pub fn save(&self, directory: &Path) -> Result<(), Box<dyn Error>> {
        self.save_colour(&directory.join(format!(
            "tile_{:06}_{:06}-colour.png",
            self.tile_index[0], self.tile_index[1]
        )))?;

        self.save_time(&directory.join(format!(
            "tile_{:06}_{:06}-time.png",
            self.tile_index[0], self.tile_index[1]
        )))
    }

    /// Save the [`Tile`] colours to a PNG file.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Tile`] cannot be encoded as a PNG file,
    /// or if the file cannot be saved.
    #[inline]
    fn save_colour(&self, file_name: &Path) -> Result<(), Box<dyn Error>> {
        let raw_samples: Vec<_> = self
            .samples
            .iter()
            .flat_map(|sample| -> [u8; 4] { sample.colour.into_format().into() })
            .collect();

        let width = self.samples.dim().1.try_into()?;
        let height = self.samples.dim().0.try_into()?;
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_samples)
            .ok_or_else(|| SaveError::new("Failed to create image buffer from raw samples."))?;

        Ok(image.save(&file_name)?)
    }

    /// Save the [`Tile`] times to a PNG file.
    ///
    /// # Errors
    ///
    /// Returns an error if the [`Tile`] cannot be encoded as a PNG file,
    /// or if the file cannot be saved.
    #[inline]
    #[allow(clippy::integer_division, clippy::cast_lossless)]
    fn save_time(&self, file_name: &Path) -> Result<(), Box<dyn Error>> {
        let raw_samples: Vec<_> = self
            .samples
            .iter()
            .flat_map(|sample| -> Result<[u8; 4], TryFromIntError> {
                let red = u8::try_from((sample.time / (64 * 64)).clamp(0, u8::MAX as u128))?;
                let green = u8::try_from((sample.time / (128 * 64)).clamp(0, u8::MAX as u128))?;
                let blue = u8::try_from((sample.time / (256 * 64)).clamp(0, u8::MAX as u128))?;
                Ok([red, green, blue, u8::MAX])
            })
            .flatten()
            .collect();

        let width = self.samples.dim().1.try_into()?;
        let height = self.samples.dim().0.try_into()?;
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_samples)
            .ok_or_else(|| SaveError::new("Failed to create image buffer from raw samples."))?;

        Ok(image.save(&file_name)?)
    }
}
