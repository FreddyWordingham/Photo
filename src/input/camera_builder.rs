//! Camera builder structure.

use serde::{Deserialize, Serialize};

/// Builds a [`Camera`] instance.
#[derive(Deserialize, Serialize)]
pub struct CameraBuilder {
    /// Observation position [x, y, z] (meters).
    position: [f64; 3],
    /// View target [x, y, z] (meters).
    target: [f64; 3],
    /// Horizontal field of view (degrees).
    field_of_view: f64,
    /// Super-samples per axis.
    super_samples_per_axis: Option<usize>,
    /// Total image resolution [height, width] (pixels).
    resolution: [usize; 2],
    /// Number of tiles along each axis [height, width].
    num_tiles: [usize; 2],
}
