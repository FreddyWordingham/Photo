use serde::{Deserialize, Serialize};

/// Runtime lighting settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingBuilder {
    /// The position of the sun. [x, y, z] (meters)
    sun_position: [f64; 3],
}

impl LightingBuilder {
    /// Construct a new instance.
    pub fn new(sun_position: [f64; 3]) -> Self {
        debug_assert!(sun_position.iter().all(|p| p.is_finite()));

        Self { sun_position }
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        self.sun_position.iter().all(|p| p.is_finite())
    }
}
