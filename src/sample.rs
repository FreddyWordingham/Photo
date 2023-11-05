use palette::Srgba;

/// Sample data for a single ray.
#[derive(Debug, Clone)]
pub struct Sample {
    pub index: (usize, usize),
    pub colour: Srgba,
}

impl Sample {
    /// Construct a new Sample object.
    pub fn new(index: (usize, usize)) -> Self {
        Self {
            index,
            colour: Srgba::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}
