use ndarray::Array3;

/// An opaque colour image.
pub struct ImageRGB<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

mod float;
mod u8;
