//! Spectrum builder structure.

use serde::{Deserialize, Serialize};

/// Builds a [`Spectrum`] instance.
#[derive(Deserialize, Serialize)]
pub struct SpectrumBuilder(Vec<u32>);
