use image::{ImageBuffer, Rgba};
use ndarray::Array2;
use palette::LinSrgba;
use std::path::Path;

use crate::render::Sample;

/// Rectangular region of the complete image.
pub struct Tile {
    pub data: Array2<Sample>,
}

impl Tile {
    /// Construct a new instance.
    pub fn new(tile_index: [usize; 2], resolution: [usize; 2], colour: LinSrgba) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);

        let offset = [tile_index[0] * resolution[0], tile_index[1] * resolution[1]];
        let data = Array2::<Sample>::from_shape_fn(resolution, |index| {
            let pixel_index = [offset[0] + index.0, offset[1] + index.1];
            Sample::new(pixel_index, colour)
        });

        Self { data }
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
