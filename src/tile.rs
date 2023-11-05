use ndarray::Array2;
use std::fmt::Display;

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

    /// Save the tile to disk.
    pub fn save(&self) {
        use image::{ImageBuffer, Rgba, RgbaImage};

        let colours = self.data.map(|sample| sample.colour);

        let (width, height) = colours.dim();
        let mut img_buffer: RgbaImage = ImageBuffer::new(width as u32, height as u32);

        for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
            let palette_pixel = colours[(x as usize, y as usize)];
            let data: [u8; 4] = palette::Srgba::into_format(palette_pixel).into();
            *pixel = Rgba(data);
        }

        let filename = format!(
            "output/tile_{:03}-{:03}.png",
            self.tile_index[0], self.tile_index[1]
        );
        img_buffer.save(filename).unwrap();
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = self.data.dim();
        for y in 0..height {
            if y > 0 {
                write!(f, "\n").unwrap();
            }
            for x in 0..width {
                write!(
                    f,
                    "{}",
                    colour_text("██", self.data[(x, height - y - 1)].colour)
                )
                .unwrap();
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
