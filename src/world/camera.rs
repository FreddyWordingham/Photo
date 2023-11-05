use nalgebra::{Unit, Vector3};

/// Converts between pixel and world coordinates.
pub struct Camera {
    /// Name of the camera.
    pub name: String,
    /// The position of the camera in world coordinates.
    pub position: Vector3<f64>,
    /// Target position of the camera in world coordinates.
    pub target: Vector3<f64>,
    /// Horizontal field of view of the camera in degrees.
    pub field_of_view: f64,
}

impl Camera {
    /// Constructs a new camera instance.
    pub fn new(
        name: String,
        position: Vector3<f64>,
        target: Vector3<f64>,
        field_of_view: f64,
    ) -> Self {
        debug_assert!(field_of_view > 0.0);

        Self {
            name,
            position,
            target,
            field_of_view,
        }
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        self.field_of_view > 0.0
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
