//! Camera structure.

use nalgebra::{Point3, Rotation3, Unit};

use crate::{engine::Engine, geometry::Ray};

/// Generates sampling rays to form an image.
pub struct Camera {
    /// Rendering engine.
    engine: Engine,
    /// Observation position (metres).
    position: Point3<f64>,
    /// View target (metres).
    look_at: Point3<f64>,
    /// Horizontal field of view (radians).
    field_of_view: f64,
    /// Super-samples per axis.
    super_samples_per_axis: usize,
    /// Resolution of each tile [height, width] (pixels).
    tile_resolution: [usize; 2],
    /// Number of tiles along each axis [height, width].
    num_tiles: [usize; 2],
}

impl Camera {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        engine: Engine,
        position: Point3<f64>,
        look_at: Point3<f64>,
        field_of_view: f64,
        super_samples_per_axis: usize,
        tile_resolution: [usize; 2],
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
            tile_resolution.iter().all(|&axis| axis > 0),
            "Tile resolution must be positive along each axis!"
        );
        debug_assert!(
            num_tiles.iter().all(|&axis| axis > 0),
            "Number of tiles must be positive along each axis!"
        );

        Self {
            engine,
            position,
            look_at,
            field_of_view,
            super_samples_per_axis,
            tile_resolution,
            num_tiles,
        }
    }

    /// Get the rendering [`Engine`] function handle.
    #[must_use]
    #[inline]
    pub const fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Get the number of samples along each axis.
    /// The total number of samples is the square of this value.
    #[must_use]
    #[inline]
    pub const fn super_samples_per_axis(&self) -> usize {
        self.super_samples_per_axis
    }

    /// Get the number of tiles along each axis [height, width].
    #[must_use]
    #[inline]
    pub const fn num_tiles(&self) -> [usize; 2] {
        self.num_tiles
    }

    /// Get the total number of tiles.
    #[must_use]
    #[inline]
    pub const fn total_num_tiles(&self) -> usize {
        self.num_tiles[0] * self.num_tiles[1]
    }

    /// Calculate the resolution of each tile.
    #[must_use]
    #[inline]
    pub const fn tile_resolution(&self) -> [usize; 2] {
        self.tile_resolution
    }

    #[must_use]
    #[inline]
    pub fn generate_ray(&self, pixel_index: [usize; 2], sub_pixel_index: [usize; 2]) -> Ray {
        debug_assert!(
            pixel_index[0] < (self.num_tiles[0] * self.tile_resolution[0]),
            "Pixel index must be within vertical resolution!"
        );
        debug_assert!(
            pixel_index[1] < (self.num_tiles[1] * self.tile_resolution[1]),
            "Pixel index must be within horizontal resolution!"
        );
        debug_assert!(
            sub_pixel_index[0] < self.super_samples_per_axis,
            "Sub-pixel index must be within vertical super-samples!"
        );
        debug_assert!(
            sub_pixel_index[1] < self.super_samples_per_axis,
            "Sub-pixel index must be within horizontal super-samples!"
        );

        let pixel = [
            pixel_index[0] as f64
                + ((sub_pixel_index[0] as f64 + 0.5) / self.super_samples_per_axis as f64),
            pixel_index[1] as f64
                + ((sub_pixel_index[1] as f64 + 0.5) / self.super_samples_per_axis as f64),
        ];

        let d_row = (pixel[0] / (self.num_tiles[0] * self.tile_resolution[0]) as f64) - 0.5;
        let d_col = (pixel[1] / (self.num_tiles[1] * self.tile_resolution[1]) as f64) - 0.5;

        let aspect_ratio = (self.num_tiles[0] * self.tile_resolution[0]) as f64
            / (self.num_tiles[1] * self.tile_resolution[1]) as f64;
        let d_theta = -d_col * (self.field_of_view * 0.5);
        let d_phi = -d_row * (self.field_of_view * aspect_ratio * 0.5);

        let forward = Unit::new_normalize(self.look_at - self.position);
        let right = Unit::new_normalize(forward.cross(&nalgebra::Vector3::z()));
        let up = Unit::new_normalize(right.cross(&forward));

        let vertical_rotation = Rotation3::from_axis_angle(&right, d_phi);
        let lateral_rotation = Rotation3::from_axis_angle(&up, d_theta);

        let direction = lateral_rotation * vertical_rotation * forward;

        Ray::new(self.position, direction)
    }
}
