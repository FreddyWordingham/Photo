use std::fmt::Display;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "valid:                         {}\n", self.is_valid())?;

        Ok(())
    }
}
