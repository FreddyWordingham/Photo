// use std::{fs::File, io::BufWriter, path::Path};

use ndarray::Array2;
// use num_traits::{Float, FromPrimitive};
// use png::{ColorType, Decoder, Encoder};

use crate::image::Image;
use crate::image_error::ImageError;

impl Image for Array2<f64> {
    fn width(&self) -> u32 {
        self.ncols() as u32
    }

    fn height(&self) -> u32 {
        self.nrows() as u32
    }
}
