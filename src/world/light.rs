//! Light source structure.

use nalgebra::Vector3;
use palette::LinSrgba;

/// Light source structure.
pub struct Light {
    /// Colour of the light.
    colour: LinSrgba,
    /// Intensity of the light.
    intensity: f64,
    /// Position of the light (meters).
    position: Vector3<f64>,
}

impl Light {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(position: Vector3<f64>, colour: LinSrgba, intensity: f64) -> Self {
        debug_assert!(intensity > 0.0, "Light intensity must be positive!");

        Self {
            colour,
            intensity,
            position,
        }
    }
}
