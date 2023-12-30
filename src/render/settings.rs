//! Runtime settings structure.

use std::path::PathBuf;

/// Runtime settings.
#[non_exhaustive]
pub struct Settings {
    /// Output directory for save files.
    pub output_directory: PathBuf,
    /// Numerical smoothing length (meters).
    pub smoothing_length: f64,
    /// Minimum weight of sampling.
    pub min_weight: f64,
    /// Maximum number of path tracing iterations.
    pub max_loops: u32,
}

impl Settings {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        output_directory: PathBuf,
        smoothing_length: f64,
        min_weight: f64,
        max_loops: u32,
    ) -> Self {
        debug_assert!(output_directory.is_dir(), "Output directory must exist!");
        debug_assert!(
            smoothing_length.is_finite(),
            "Smoothing length must be finite!"
        );
        debug_assert!(smoothing_length > 0.0, "Smoothing length must be positive!");
        debug_assert!(
            (0.0..=1.0).contains(&min_weight),
            "Minimum weight must be in the range [0.0, 1.0]!"
        );

        Self {
            output_directory,
            smoothing_length,
            min_weight,
            max_loops,
        }
    }
}
