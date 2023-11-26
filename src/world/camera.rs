use nalgebra::{Point3, Rotation3, Unit};

use crate::geometry::Ray;

pub struct Camera {
    position: Point3<f64>,
    target: Point3<f64>,
    field_of_view: f64,
    super_samples_per_axis: usize,
    resolution: [usize; 2],
    num_tiles: [usize; 2],
}

impl Camera {
    pub fn new(
        position: Point3<f64>,
        target: Point3<f64>,
        field_of_view: f64,
        super_samples_per_axis: usize,
        resolution: [usize; 2],
        num_tiles: [usize; 2],
    ) -> Self {
        debug_assert!(position.iter().all(|p| p.is_finite()));
        debug_assert!(target.iter().all(|t| t.is_finite()));
        debug_assert!(
            position
                .iter()
                .zip(target.iter())
                .fold(0.0, |acc, (p, t)| { acc + (p - t).abs() })
                > 0.0
        );
        debug_assert!(field_of_view > 0.0);
        debug_assert!(super_samples_per_axis > 0);
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);
        debug_assert!(num_tiles[0] > 0);
        debug_assert!(num_tiles[1] > 0);
        debug_assert!(resolution[0] % num_tiles[0] == 0);
        debug_assert!(resolution[1] % num_tiles[1] == 0);

        Self {
            position,
            target,
            field_of_view,
            super_samples_per_axis,
            resolution,
            num_tiles,
        }
    }

    pub fn super_samples_per_axis(&self) -> usize {
        self.super_samples_per_axis
    }

    pub fn num_tiles(&self) -> [usize; 2] {
        self.num_tiles
    }

    pub fn tile_resolution(&self) -> [usize; 2] {
        [
            self.resolution[0] / self.num_tiles[0],
            self.resolution[1] / self.num_tiles[1],
        ]
    }

    pub fn generate_ray(&self, pixel: [usize; 2], sub_pixel: [usize; 2]) -> Ray {
        debug_assert!(pixel[0] < self.resolution[0]);
        debug_assert!(pixel[1] < self.resolution[1]);
        debug_assert!(sub_pixel[0] < self.super_samples_per_axis);
        debug_assert!(sub_pixel[1] < self.super_samples_per_axis);

        let pixel = [
            pixel[0] as f64 + ((sub_pixel[0] as f64 + 0.5) / self.super_samples_per_axis as f64),
            pixel[1] as f64 + ((sub_pixel[1] as f64 + 0.5) / self.super_samples_per_axis as f64),
        ];

        let d_row = (pixel[0] / self.resolution[0] as f64) - 0.5;
        let d_col = (pixel[1] / self.resolution[1] as f64) - 0.5;

        let d_theta = d_col * (self.field_of_view * 0.5);
        let d_phi = -d_row
            * (self.field_of_view * (self.resolution[0] as f64 / self.resolution[1] as f64) * 0.5);

        let forward = Unit::new_normalize(self.target - self.position);
        let right = Unit::new_normalize(forward.cross(&nalgebra::Vector3::z()));
        let up = Unit::new_normalize(right.cross(&forward));

        let vertical_rotation = Rotation3::from_axis_angle(&right, d_phi);
        let lateral_rotation = Rotation3::from_axis_angle(&up, d_theta);

        let direction = lateral_rotation * vertical_rotation * forward;

        Ray::new(self.position, direction)
    }
}
