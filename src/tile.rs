use ndarray::prelude::*;

use crate::Sample;

/// A tile is a rectangular region of the total image.
pub struct Tile {
    /// Index of the tile in the image. [row, column]
    pub tile_index: [usize; 2],
    /// Tile pixel data. [rows, columns]
    pub data: Array2<Sample>,
}

impl Tile {
    /// Construct a new Tile object.
    pub fn new(tile_index: [usize; 2], resolution: [usize; 2]) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        Self {
            tile_index,
            data: Array2::<Sample>::from_shape_fn(resolution, |sample_index| {
                Sample::new(sample_index.into())
            }),
        }
    }

    pub fn save(&self, output_directory: &std::path::Path) {
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
        let image = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
            self.data.dim().1 as u32,
            self.data.dim().0 as u32,
            raw_data,
        )
        .unwrap();

        image.save(image_path).unwrap();
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rows, columns) = self.data.dim();
        for row in 0..rows {
            writeln!(f, "").unwrap();
            for column in 0..columns {
                write!(f, "{}", colour_text("██", self.data[(row, column)].colour)).unwrap();
            }
        }

        Ok(())
    }
}

fn colour_text(text: &str, color: palette::Srgba) -> String {
    let rgba: [u8; 4] = palette::Srgba::into_format(color).into();
    format!(
        "\x1B[38;2;{};{};{}m{}\x1B[0m",
        rgba[0], rgba[1], rgba[2], text
    )
}
