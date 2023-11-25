use image::{ImageBuffer, Rgba};
use ndarray::Array2;
use std::path::Path;

use crate::render::Sample;

/// Rectangular region of the complete image.
pub struct Tile {
    pub data: Array2<Sample>,
}

impl Tile {
    /// Construct a new instance.
    pub fn new(resolution: [usize; 2]) -> Self {
        Self {
            data: Array2::default(resolution),
        }
    }

    pub fn save(&self, file_name: &Path) {
        let raw_data: Vec<_> = self
            .data
            .iter()
            .flat_map(|sample| {
                let raw: [u8; 4] = sample.colour.into_format().into();
                raw
            })
            .collect();

        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(
            self.data.dim().1 as u32,
            self.data.dim().0 as u32,
            raw_data,
        )
        .unwrap();

        image.save(&file_name).unwrap();
    }
}
