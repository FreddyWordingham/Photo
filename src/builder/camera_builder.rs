//! [`Camera`] builder structure.

use serde::{Deserialize, Serialize};

use crate::{
    builder::{EffectBuilder, EngineBuilder},
    error::ValidationError,
    world::Camera,
};

/// Builds a [`Camera`] instance.
#[derive(Deserialize, Serialize)]
pub struct CameraBuilder {
    /// Rendering engine function builder.
    engine: EngineBuilder,
    /// Post-processing effects.
    effects: Option<Vec<EffectBuilder>>,
    /// Observation position [x, y, z] (meters).
    position: [f64; 3],
    /// View target [x, y, z] (meters).
    look_at: [f64; 3],
    /// Horizontal field of view (degrees).
    field_of_view: f64,
    /// Super-samples per axis.
    super_samples_per_axis: Option<usize>,
    /// Total image resolution [width, height] (pixels).
    resolution: [usize; 2],
    /// Number of tiles along each axis [width, height].
    num_tiles: [usize; 2],
}

impl CameraBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the engine is not valid,
    /// or if the position is not finite,
    /// or if the look-at position is not finite,
    /// or if the field of view is not finite, or not positive,
    /// or if the super-samples per axis is not positive, if it is specified,
    /// or if the resolution is not positive along each axis,
    /// or if the number of tiles is not positive along each axis.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.position.iter().all(|component| component.is_finite()) {
            return Err(ValidationError::new(&format!(
                "Camera observation position must be finite, but the values are [{} {} {}]!",
                self.position[0], self.position[1], self.position[2]
            )));
        }

        if !self.look_at.iter().all(|component| component.is_finite()) {
            return Err(ValidationError::new(&format!(
                "Camera look-at position must be finite, but the values are [{} {} {}]!",
                self.look_at[0], self.look_at[1], self.look_at[2]
            )));
        }

        if !self.field_of_view.is_finite() {
            return Err(ValidationError::new(&format!(
                "Camera field of view must be finite, but the value is {}!",
                self.field_of_view
            )));
        }
        if self.field_of_view < 0.0 {
            return Err(ValidationError::new(&format!(
                "Camera field of view must be positive, but the value is {}!",
                self.field_of_view
            )));
        }

        if let Some(super_samples_per_axis) = self.super_samples_per_axis {
            if super_samples_per_axis == 0 {
                return Err(ValidationError::new(&format!("Camera super-samples per axis must be positive, but the value is {super_samples_per_axis}!")));
            }
        }

        if !self.resolution.iter().all(|component| component > &0) {
            return Err(ValidationError::new(&format!(
                "Camera resolution must be greater than zero along each axis, but the values are [{} {}]!",
                self.resolution[0], self.resolution[1]
            )));
        }
        if self.resolution[0] % self.num_tiles[0] != 0 {
            return Err(ValidationError::new(&format!(
                "Camera resolution width must be divisible by the number of tiles width, but the values are {} and {}!",
                self.resolution[0], self.num_tiles[0]
            )));
        }
        if self.resolution[1] % self.num_tiles[1] != 0 {
            return Err(ValidationError::new(&format!(
                "Camera resolution height must be divisible by the number of tiles height, but the values are {} and {}!",
                self.resolution[1], self.num_tiles[1]
            )));
        }

        if !self.num_tiles.iter().all(|component| component > &0) {
            return Err(ValidationError::new(&format!(
                "Number of camera tiles must be greater than zero along each axis, but the values are [{} {}]!",
                self.num_tiles[0], self.num_tiles[1]
            )));
        }

        Ok(())
    }

    /// Build a [`Camera`] instance.
    #[must_use]
    #[inline]
    #[allow(clippy::integer_division)]
    pub fn build(&self) -> Camera {
        Camera::new(
            self.engine.build(),
            self.effects.as_ref().map_or(Vec::new(), |effects| {
                effects.iter().map(|effect| effect.build()).collect()
            }),
            self.position.into(),
            self.look_at.into(),
            self.field_of_view.to_radians(),
            self.super_samples_per_axis.unwrap_or(1),
            [
                self.resolution[1] / self.num_tiles[1],
                self.resolution[0] / self.num_tiles[0],
            ],
            [self.num_tiles[1], self.num_tiles[0]],
        )
    }
}
