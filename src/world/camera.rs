//! Camera structure.

use nalgebra::Vector3;

/// Generates sampling rays to form an image.
pub struct Camera {
    /// Observation position.
    position: Vector3<f64>,
    /// View target.
    target: Vector3<f64>,
    /// Horizontal field of view (radians).
    field_of_view: f64,
    /// Super-samples per axis.
    super_samples_per_axis: usize,
    /// Total image resolution (pixels) [height, width].
    resolution: [usize; 2],
    /// Number of tiles along each axis [height, width].
    num_tiles: [usize; 2],
}
