use ndarray::Array3;

/// A colour image with transparency.
pub struct ImageRGBA<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

mod float;
mod u8;
