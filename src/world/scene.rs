use std::fmt::{Display, Formatter, Result};

/// Scene to be rendered.
pub struct Scene {}

impl Scene {
    /// Construct a new Scene object.
    pub fn new() -> Self {
        Self {}
    }

    /// Check that the current combination of values is valid.
    pub fn is_valid(&self) -> bool {
        true
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "valid:                         {}", self.is_valid())
    }
}
