use nalgebra::Vector3;
use std::f64::consts::PI;

use crate::world::Camera;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CameraSettings {
    /// Name of the camera.
    pub name: String,
    /// The position of the camera. [x, y, z]
    pub position: [f64; 3],
    /// The target of the camera. [x, y, z]
    pub target: [f64; 3],
    /// Horizontal field of view of the camera. [degrees].
    pub field_of_view: f64,
}

impl CameraSettings {
    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        self.field_of_view > 0.0 && self.field_of_view < 180.0
    }

    /// Build a camera from the current settings.
    pub fn build_camera(&self) -> Camera {
        crate::world::Camera::new(
            self.name.clone(),
            Vector3::from_row_slice(&self.position),
            Vector3::from_row_slice(&self.target),
            self.field_of_view * PI / 180.0,
        )
    }
}
