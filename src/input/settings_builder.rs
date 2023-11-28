use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::render::Settings;

/// Input settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsBuilder {
    output_directory: PathBuf,
    sun_position: [f64; 3],
    smoothing_length: f64,
    min_weight: f64,
    max_loops: u16,
    mesh_bvh_max_children: usize,
    scene_bvh_max_children: usize,
}

impl SettingsBuilder {
    /// Construct a new instance.
    pub fn new(
        output_directory: &str,
        sun_position: [f64; 3],
        smoothing_length: f64,
        min_weight: f64,
        max_loops: u16,
        mesh_bvh_max_children: usize,
        scene_bvh_max_children: usize,
    ) -> Self {
        let new = Self {
            output_directory: output_directory.into(),
            sun_position,
            smoothing_length,
            min_weight,
            max_loops,
            mesh_bvh_max_children,
            scene_bvh_max_children,
        };

        debug_assert!(new.is_valid());

        new
    }

    /// Check if the settings parameters are valid.
    pub fn is_valid(&self) -> bool {
        self.output_directory.is_dir()
            && self.sun_position.iter().all(|&x| x.is_finite())
            && self.smoothing_length > 0.0
            && self.min_weight.is_sign_positive()
            && self.max_loops > 0
            && self.mesh_bvh_max_children >= 2
            && self.scene_bvh_max_children >= 2
    }

    /// Build the settings.
    pub fn build(&self) -> Settings {
        Settings::new(
            &self.output_directory,
            self.sun_position.into(),
            self.smoothing_length,
            self.min_weight,
            self.max_loops,
            self.mesh_bvh_max_children,
            self.scene_bvh_max_children,
        )
    }
}
