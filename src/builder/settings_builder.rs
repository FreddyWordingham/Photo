//! Settings builder structure.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{error::ValidationError, render::Settings};

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
    max_loops: u32,
    /// Target maximum number of [`Triangle`] per [`Bvh`] node for [`Mesh`]es.
    mesh_bvh_max_children: usize,
    /// Maximum tree depth for [`Mesh`] [`Bvh`]s.
    mesh_bvh_max_depth: usize,
    /// Target maximum number of [`Entity`]s per [`Bvh`] node for [`Scene`]s.
    scene_bvh_max_children: usize,
    /// Maximum tree depth for [`Entity`] [`Bvh`]s.
    scene_bvh_max_depth: usize,
}

impl SettingsBuilder {
    /// Check if the build parameters are valid.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationError`] if the output directory does not exist,
    /// or if the smoothing length is not finite or positive,
    /// or if the minimum weight is not in the range `[0.0, 1.0]`.
    #[inline]
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !self.output_directory.is_dir() {
            return Err(ValidationError::new(
                "Settings output directory must already exist, but it does not!",
            ));
        }

        if !self.smoothing_length.is_finite() {
            return Err(ValidationError::new(&format!(
                "Settings smoothing length muse be finite, but value is {}!",
                self.smoothing_length
            )));
        }
        if self.smoothing_length <= 0.0 {
            return Err(ValidationError::new(&format!(
                "Settings smoothing length must be positive, but the value is {}!",
                self.smoothing_length
            )));
        }

        if !(0.0..=1.0).contains(&self.min_weight) {
            return Err(ValidationError::new(&format!(
                "Settings minimum weight must be in the range [0.0, 1.0], but the value is {}!",
                self.min_weight
            )));
        }

        if self.mesh_bvh_max_children <= 1 {
            return Err(ValidationError::new(&format!(
                "Mesh BVH max children must be at least 2, but the value is {}!",
                self.mesh_bvh_max_children
            )));
        }

        if self.mesh_bvh_max_depth == 0 {
            return Err(ValidationError::new(&format!(
                "Mesh BVH max depth must be positive, but the value is {}!",
                self.mesh_bvh_max_depth
            )));
        }

        if self.scene_bvh_max_children <= 1 {
            return Err(ValidationError::new(&format!(
                "Scene BVH max children must be at least 2, but the value is {}!",
                self.scene_bvh_max_children
            )));
        }

        if self.scene_bvh_max_depth == 0 {
            return Err(ValidationError::new(&format!(
                "Scene BVH max depth must be positive, but the value is {}!",
                self.scene_bvh_max_depth
            )));
        }

        Ok(())
    }

    /// Build a [`Settings`] instance.
    #[must_use]
    #[inline]
    pub fn build(&self) -> Settings {
        Settings::new(
            self.output_directory.clone(),
            self.smoothing_length,
            self.min_weight,
            self.max_loops,
            self.mesh_bvh_max_children,
            self.mesh_bvh_max_depth,
            self.scene_bvh_max_children,
            self.scene_bvh_max_depth,
        )
    }
}
