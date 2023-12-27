//! Light source builder structure.

use serde::{Deserialize, Serialize};

use crate::{error::ValidationError, utility::colour::from_u32, world::Light};

/// Builds a [`Light`] instance.
#[derive(Deserialize, Serialize)]
pub struct LightBuilder {
    /// Position of the light [x, y, z] (meters).
    position: [f64; 3],
    /// Colour of the light.
    colour: u32,
    /// Intensity of the light.
    intensity: f64,
}

impl LightBuilder {
    /// Validate the builder.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the position is not finite,
    /// or if the intensity is not positive.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.position.iter().all(|component| component.is_finite()) {
            return Err(ValidationError::new(&format!(
                "Light position must be finite, but the values are [{} {} {}]!",
                self.position[0], self.position[1], self.position[2]
            )));
        }

        if !(0.0..=1.0).contains(&self.intensity) {
            return Err(ValidationError::new(&format!(
                "Light intensity must be in the range [0.0, 1.0], but it is {}!",
                self.intensity
            )));
        }

        Ok(())
    }

    /// Build a [`Light`] instance.
    #[must_use]
    #[inline]
    pub fn build(&self) -> Light {
        Light::new(self.position.into(), from_u32(self.colour), self.intensity)
    }
}
