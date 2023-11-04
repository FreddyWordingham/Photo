use std::fmt::Display;

/// Sample data for a single ray.
#[derive(Debug, Clone)]
pub struct Sample {
    pub index: (usize, usize),
    pub total: f64,
}

impl Sample {
    /// Construct a new Sample object.
    pub fn new(index: (usize, usize)) -> Self {
        Self { index, total: 0.0 }
    }
}

impl Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.total)
    }
}
