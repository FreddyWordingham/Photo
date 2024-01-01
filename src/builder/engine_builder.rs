//! [`Engine`] builder structure.

use nalgebra::Point3;
use serde::{Deserialize, Serialize};

use crate::{engine, engine::Engine, error::ValidationError};

/// Parametrises an [`engine`] function.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum EngineBuilder {
    /// X-ray.
    Xray,
    /// Stencil.
    Stencil,
    /// Distance.
    Distance(f64),
    /// Surface normal.
    Normal,
    /// Ambient lighting.
    Ambient,
    /// Full [`Ray`]-traced lighting.
    Full([f64; 3]),
    /// Occlusion engine.
    Occlusion([f64; 3]),
    /// Test engine.
    Test([f64; 3]),
    /// Diffuse lighting.
    Diffuse(([f64; 3], f64)),
    /// Mesh side.
    Side(([f64; 3], f64)),
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
            Self::Xray | Self::Stencil | Self::Normal | Self::Ambient => Ok(()),
            Self::Distance(width) => {
                if !width.is_finite() {
                    return Err(ValidationError::new(&format!(
                        "Engine-Distance: parameter must be finite, but the value is {width}!",
                    )));
                }
                if *width < 0.0 {
                    return Err(ValidationError::new(&format!(
                        "Engine-Distance: parameter must be positive, but the value is {width}!",
                    )));
                }
                Ok(())
            }
            Self::Full(sun_position) | Self::Occlusion(sun_position) | Self::Test(sun_position) => {
                if !sun_position.iter().all(|&x| x.is_finite()) {
                    return Err(ValidationError::new(&format!(
                        "Engine-Ambient sun position must be finite, but the value is {sun_position:?}!"
                    )));
                }
                Ok(())
            }
            Self::Diffuse((sun_position, max_shadow_distance))
            | Self::Side((sun_position, max_shadow_distance)) => {
                if !sun_position.iter().all(|&x| x.is_finite()) {
                    return Err(ValidationError::new(&format!(
                        "Engine-Sun position must be finite, but the value is {sun_position:?}!"
                    )));
                }
                if !max_shadow_distance.is_finite() {
                    return Err(ValidationError::new(&format!(
                        "Engine-Max shadow distance must be finite, but the value is {max_shadow_distance}!"
                    )));
                }
                if *max_shadow_distance <= 0.0 {
                    return Err(ValidationError::new(&format!(
                        "Engine-Max shadow distance must be positive, but the value is {max_shadow_distance}!"
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
            Self::Xray => Box::new(engine::xray),
            Self::Stencil => Box::new(engine::stencil),
            Self::Distance(distance) => Box::new(move |settings, scene, ray| {
                engine::distance(settings, scene, ray, distance)
            }),
            Self::Normal => Box::new(engine::normal),
            Self::Ambient => Box::new(engine::ambient),
            Self::Diffuse((sun_position, max_shadow_distance)) => {
                Box::new(move |settings, scene, ray| {
                    engine::diffuse(
                        settings,
                        scene,
                        ray,
                        &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                        max_shadow_distance,
                    )
                })
            }
            Self::Full(sun_position) => Box::new(move |settings, scene, ray| {
                engine::full(
                    settings,
                    scene,
                    ray,
                    0,
                    1.0,
                    1.0,
                    &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                )
            }),
            Self::Occlusion(sun_position) => Box::new(move |settings, scene, ray| {
                engine::occlusion(
                    settings,
                    scene,
                    ray,
                    0,
                    1.0,
                    1.0,
                    &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                )
            }),
            Self::Test(sun_position) => Box::new(move |settings, scene, ray| {
                engine::test(
                    settings,
                    scene,
                    ray,
                    0,
                    1.0,
                    1.0,
                    &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                )
            }),
            Self::Side((sun_position, max_shadow_distance)) => {
                Box::new(move |settings, scene, ray| {
                    engine::side(
                        settings,
                        scene,
                        ray,
                        &Point3::new(sun_position[0], sun_position[1], sun_position[2]),
                        max_shadow_distance,
                    )
                })
            }
        }
    }
}
