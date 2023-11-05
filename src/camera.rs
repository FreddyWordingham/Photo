use nalgebra::{Unit, Vector3};

/// Converts between pixel and world coordinates.
pub struct Camera {
    /// The position of the camera in world coordinates.
    pub position: Vector3<f64>,
    /// Target position of the camera in world coordinates.
    pub target: Vector3<f64>,
}

impl Camera {
    /// Constructs a new camera instance.
    pub fn new(position: Vector3<f64>, target: Vector3<f64>) -> Self {
        Self { position, target }
    }

    /// Get the forwards direction of the camera.
    pub fn forwards(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.target - self.position)
    }

    /// Get the right direction of the camera.
    pub fn right(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(Vector3::z_axis().cross(&self.forwards()))
    }

    /// Get the up direction of the camera.
    pub fn up(&self) -> Unit<Vector3<f64>> {
        Unit::new_normalize(self.forwards().cross(&self.right()))
    }
}
