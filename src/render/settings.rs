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
    /// Target maximum number of [`Triangle`] per [`Bvh`] node for [`Mesh`]es.
    pub mesh_bvh_max_children: usize,
    /// Maximum tree depth for [`Mesh`] [`Bvh`]s.
    pub mesh_bvh_max_depth: usize,
    /// Target maximum number of [`Entity`]s per [`Bvh`] node for [`Scene`]s.
    pub scene_bvh_max_children: usize,
    /// Maximum tree depth for [`Entity`] [`Bvh`]s.
    pub scene_bvh_max_depth: usize,
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
        mesh_bvh_max_children: usize,
        mesh_bvh_max_depth: usize,
        scene_bvh_max_children: usize,
        scene_bvh_max_depth: usize,
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
        debug_assert!(
            mesh_bvh_max_children >= 2,
            "Mesh BVH max children must be at least 2!"
        );
        debug_assert!(
            mesh_bvh_max_depth > 0,
            "Mesh BVH max depth must be positive!"
        );
        debug_assert!(
            scene_bvh_max_children >= 2,
            "Scene BVH max children must be at least 2!"
        );
        debug_assert!(
            scene_bvh_max_depth > 0,
            "Scene BVH max depth must be positive!"
        );

        Self {
            output_directory,
            smoothing_length,
            min_weight,
            max_loops,
            mesh_bvh_max_children,
            mesh_bvh_max_depth,
            scene_bvh_max_children,
            scene_bvh_max_depth,
        }
    }
}
