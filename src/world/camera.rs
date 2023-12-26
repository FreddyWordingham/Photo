//! Camera structure.

use nalgebra::Vector3;

/// Generates sampling rays to form an image.
pub struct Camera {
    /// Observation position (metres).
    position: Vector3<f64>,
    /// View target (metres).
    look_at: Vector3<f64>,
    /// Horizontal field of view (radians).
    field_of_view: f64,
    /// Super-samples per axis.
    super_samples_per_axis: usize,
    /// Total image resolution [width, height] (pixels).
    resolution: [usize; 2],
    /// Number of tiles along each axis [width, height].
    num_tiles: [usize; 2],
}
