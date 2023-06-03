use ndarray::Array2;
use palette::LinSrgba;

pub struct Image {
    pub data: Array2<LinSrgba>,
}

impl Image {
    /// Construct a new image.
    pub fn new(width: usize, height: usize) -> Self {
        let data = Array2::default((width, height));
        Self { data }
    }

    /// Get the resolution of the image.
    pub fn resolution(&self) -> (usize, usize) {
        let shape = self.data.shape();
        (shape[0], shape[1])
    }
}
