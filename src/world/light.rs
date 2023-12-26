//! Light source structure.

use nalgebra::Vector3;
use palette::LinSrgba;

/// Light source structure.
pub struct Light {
    /// Colour of the light.
    colour: LinSrgba,
    /// Intensity of the light.
    intensity: f64,
    /// Position of the light.
    position: Vector3<f64>,
}
