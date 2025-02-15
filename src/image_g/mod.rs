use ndarray::Array2;

/// An opaque grayscale image.
pub struct ImageG<T> {
    /// Image data stored in row-major order.
    pub data: Array2<T>,
}

mod float;
mod u8;
