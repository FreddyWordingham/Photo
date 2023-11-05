/// Sample data for a single ray.
#[derive(Debug, Clone)]
pub struct Sample {
    pub sample_index: [usize; 2],
    pub colour: palette::Srgba,
}

impl Sample {
    /// Construct a new Sample object.
    pub fn new(sample_index: [usize; 2]) -> Self {
        Self {
            colour: palette::Srgba::new(0.0, 0.0, 0.0, 1.0),
            sample_index,
        }
    }
}
