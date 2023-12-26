//! Settings builder structure.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Builds a [`Settings`] instance.
#[derive(Deserialize, Serialize)]
pub struct SettingsBuilder {
    /// Output directory for save files.
    output_directory: PathBuf,
    /// Numerical smoothing length (meters).
    smoothing_length: f64,
    /// Minimum weight of sampling.
    min_weight: f64,
    /// Maximum number of path tracing iterations.
    max_loops: u16,
}
