use ndarray::Array3;

/// A grayscale image with transparency.
pub struct ImageGA<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

mod float;
mod u8;
