use serde::{Deserialize, Serialize};

use crate::assets::Gradient;

/// Colour gradient parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientBuilder {
    colours: Vec<[f32; 4]>,
}

impl GradientBuilder {
    /// Construct a new instance.
    pub fn new(colours: Vec<[f32; 4]>) -> Self {
        let new = Self { colours };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the gradient parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.colours.len() > 0
            && self
                .colours
                .iter()
                .all(|colour| colour.iter().all(|&c| c >= 0.0 && c <= 1.0))
    }

    /// Build the gradient.
    pub fn build(&self) -> Gradient {
        debug_assert!(self.is_valid());

        Gradient::new(self.colours.clone())
    }
}
