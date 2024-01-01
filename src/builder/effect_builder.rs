//! [`Effect`] builder structure.

use serde::{Deserialize, Serialize};

use crate::{effects, effects::Effect, error::ValidationError};

/// Parametrises a post-processing [`Effect`] function.
#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum EffectBuilder {
    /// Outline.
    Outline,
    /// Overlaid outline.
    OutlineOverlay,
}

impl EffectBuilder {
    /// Check if the build parameters are all valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the [`Effect`] configuration is invalid.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Self::Outline | Self::OutlineOverlay => Ok(()),
        }
    }

    /// Build a [`Effect`] function handle.
    #[must_use]
    #[inline]
    pub fn build(&self) -> Effect {
        match *self {
            Self::Outline => Box::new(|tile| effects::outline(tile, false)),
            Self::OutlineOverlay => Box::new(|tile| effects::outline(tile, true)),
        }
    }
}
