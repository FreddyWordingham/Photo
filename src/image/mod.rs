use ndarray::Array2;

/// An image with a complete pixel in each element.
pub struct Image<T> {
    /// Image data stored in row-major order.
    pub data: Array2<T>,
}

mod lin_srgb;
mod lin_srgba;
