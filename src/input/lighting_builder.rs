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
        let lighting_builder = Self { sun_position };

        debug_assert!(lighting_builder.is_valid());

        lighting_builder
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        if !self.sun_position.iter().all(|p| p.is_finite()) {
            println!("INVALID! Invalid sun position: {:?}", self.sun_position);
            return false;
        }

        true
    }
}
