//! Material builder structure.

use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::{engine, engine::Engine, error::ValidationError};

/// Parametrises an [`engine`] function.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum EngineBuilder {
    /// Stencil.
    Stencil,
    /// Distance.
    Distance(f64),
    /// Surface normal.
    Normal,
    /// Surface [`Material`].
    Material,
    /// Ambient lighting.
    Ambient([f64; 3]),
}

impl EngineBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the [`engine`] configuration is invalid.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Self::Stencil => Ok(()),
            Self::Distance(width) => {
                if !width.is_finite() {
                    return Err(ValidationError::new(&format!(
                        "Engine-Distance: parameter must be finite, but the value is {}!",
                        width
                    )));
                }
                if *width < 0.0 {
                    return Err(ValidationError::new(&format!(
                        "Engine-Distance: parameter must be positive, but the value is {}!",
                        width
                    )));
                }

                Ok(())
            }
            Self::Normal => Ok(()),
            Self::Material => Ok(()),
            Self::Ambient(sun_position) => {
                if !sun_position.iter().all(|&x| x.is_finite()) {
                    return Err(ValidationError::new(&format!(
                        "Engine-Ambient sun position must be finite, but the value is {:?}!",
                        sun_position
                    )));
                }

                Ok(())
            }
        }
    }

    /// Build a [`engine`] function handle.
    #[must_use]
    #[inline]
    pub fn build(&self) -> Engine {
        match *self {
            Self::Stencil => {
                Box::new(|scene, pixel_index, ray| engine::stencil(scene, pixel_index, ray))
            }
            Self::Distance(distance) => Box::new(move |scene, pixel_index, ray| {
                engine::distance(scene, pixel_index, ray, distance)
            }),
            Self::Normal => {
                Box::new(|scene, pixel_index, ray| engine::normal(scene, pixel_index, ray))
            }
            Self::Material => {
                Box::new(|scene, pixel_index, ray| engine::material(scene, pixel_index, ray))
            }
            Self::Ambient(sun_position) => Box::new(move |scene, pixel_index, ray| {
                engine::ambient(
                    scene,
                    pixel_index,
                    ray,
                    &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                )
            }),
        }
    }
}
