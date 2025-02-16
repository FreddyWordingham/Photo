use ndarray::{s, Array2, Axis};

/// An image with a complete pixel in each element.
#[derive(Debug, Clone)]
pub struct Image<T> {
    /// Image data stored in row-major order.
    pub data: Array2<T>,
}

impl<T: Clone> Image<T> {
    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    /// Transposes the image.
    pub fn transpose(&mut self) {
        self.data = self.data.t().to_owned();
    }

    /// Flips the image vertically.
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    /// Flips the image horizontally.
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Rotates the image 90 degrees clockwise (right).
    ///
    /// For square images, the rotation is done in-place for performance.
    /// For non-square images, a new array is allocated.
    pub fn rotate_clockwise(&mut self) {
        self.data = self.data.t().slice(s![.., ..;-1]).to_owned();
    }

    /// Rotates the image 90 degrees anticlockwise (left).
    pub fn rotate_anticlockwise(&mut self) {
        self.data = self.data.t().slice(s![..;-1, ..]).to_owned();
    }

    /// Rotates the image 180 degrees.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
    }
}

mod lin_srgb;
mod lin_srgba;
