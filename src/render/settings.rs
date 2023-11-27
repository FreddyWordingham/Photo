use nalgebra::Point3;
use std::path::{Path, PathBuf};

/// Input settings.
pub struct Settings {
    output_directory: PathBuf,
    sun_position: Point3<f64>,
    min_weight: f64,
    max_loops: u16,
    mesh_bvh_max_children: usize,
    scene_bvh_max_children: usize,
}

impl Settings {
    pub fn new(
        output_directory: &Path,
        sun_position: Point3<f64>,
        min_weight: f64,
        max_loops: u16,
        mesh_bvh_max_children: usize,
        scene_bvh_max_children: usize,
    ) -> Self {
        debug_assert!(sun_position.iter().all(|&x| x.is_finite()));
        debug_assert!(min_weight > 0.0);
        debug_assert!(max_loops > 0);
        debug_assert!(mesh_bvh_max_children >= 2);
        debug_assert!(scene_bvh_max_children >= 2);

        Self {
            output_directory: output_directory.to_path_buf(),
            sun_position,
            min_weight,
            max_loops,
            mesh_bvh_max_children,
            scene_bvh_max_children,
        }
    }

    pub fn output_directory(&self) -> &Path {
        &self.output_directory
    }

    pub fn sun_position(&self) -> &Point3<f64> {
        &self.sun_position
    }

    pub fn min_weight(&self) -> f64 {
        self.min_weight
    }

    pub fn max_loops(&self) -> u16 {
        self.max_loops
    }

    pub fn mesh_bvh_max_children(&self) -> usize {
        self.mesh_bvh_max_children
    }

    pub fn scene_bvh_max_children(&self) -> usize {
        self.scene_bvh_max_children
    }
}
