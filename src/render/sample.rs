use palette::LinSrgba;

#[derive(Debug, Clone)]
pub struct Sample {
    pub pixel_index: [usize; 2],
    pub colour: LinSrgba,
}

impl Sample {
    /// Construct a new instance.
    pub fn new(pixel_index: [usize; 2], colour: LinSrgba) -> Self {
        Self {
            pixel_index,
            colour,
        }
    }
}
