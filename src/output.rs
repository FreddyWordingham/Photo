use ndarray::Array2;

use crate::sample::Sample;

pub struct Output {
    /// Array of samples.
    pub data: Array2<Sample>,
}

impl Output {
    /// Construct a new Output object.
    pub fn new() -> Self {
        Self {
            data: Array2::<Sample>::zeros((0, 0)),
        }
    }
}
