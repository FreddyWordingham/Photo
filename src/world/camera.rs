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

impl Camera {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        position: Vector3<f64>,
        look_at: Vector3<f64>,
        field_of_view: f64,
        super_samples_per_axis: usize,
        resolution: [usize; 2],
        num_tiles: [usize; 2],
    ) -> Self {
        debug_assert!(
            position != look_at,
            "Camera position and look-at must not be equal!"
        );
        debug_assert!(field_of_view.is_finite(), "Field of view must be finite!");
        debug_assert!(field_of_view > 0.0, "Field of view must be positive!");
        debug_assert!(
            super_samples_per_axis > 0,
            "Super-samples per axis must be positive!"
        );
        debug_assert!(
            resolution.iter().all(|&axis| axis > 0),
            "Resolution must be positive along each axis!"
        );
        debug_assert!(
            num_tiles.iter().all(|&axis| axis > 0),
            "Number of tiles must be positive along each axis!"
        );

        Self {
            position,
            look_at,
            field_of_view,
            super_samples_per_axis,
            resolution,
            num_tiles,
        }
    }
}
