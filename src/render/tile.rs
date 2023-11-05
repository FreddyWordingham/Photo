use image::{ImageBuffer, Rgba};
use ndarray::prelude::*;
use std::{
    fmt::{Display, Formatter, Result},
    path::Path,
};

use crate::{render::Sample, utility::terminal};

/// A tile is a rectangular region of the total image.
pub struct Tile {
    /// Index of the tile in the image. [row, column]
    pub tile_index: [usize; 2],
    /// Tile pixel data. [rows, columns]
    pub data: Array2<Sample>,
}

impl Tile {
    /// Construct a new Tile object.
    pub fn _new(tile_index: [usize; 2], resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        Self {
            tile_index,
            data: Array2::<Sample>::from_shape_fn(resolution, |sample_index| {
                Sample::_new(sample_index.into())
            }),
        }
    }

    pub fn _save(&self, output_directory: &Path) {
        let image_name = format!(
            "tile_{:03}_{:03}.png",
            self.tile_index[0], self.tile_index[1]
        );
        let image_path = output_directory.join(image_name);

        let raw_data: Vec<_> = self
            .data
            .iter()
            .flat_map(|sample| {
                let raw: [u8; 4] = sample.colour.into_format().into();
                raw
            })
            .collect();

        // Write image with image library.
        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.data.dim().1 as u32,
            self.data.dim().0 as u32,
            raw_data,
        )
        .unwrap();

        image.save(image_path).unwrap();
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let (rows, columns) = self.data.dim();
        for row in 0..rows {
            writeln!(f, "").unwrap();
            for column in 0..columns {
                write!(
                    f,
                    "{}",
                    terminal::colour_text("██", self.data[(row, column)].colour)
                )
                .unwrap();
            }
        }

        Ok(())
    }
}
