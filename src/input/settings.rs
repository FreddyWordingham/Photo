use crate::input::CameraSettings;

/// Runtime rendering settings.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// The resolution of the image in pixels. [rows, columns]
    resolution: [usize; 2],
    /// The resolution of each tile in pixels. [rows, columns]
    tile_resolution: [usize; 2],
    /// List of cameras to render from.
    cameras: Vec<CameraSettings>,
}

impl Settings {
    /// Construct a new Settings object.
    pub fn new(
        resolution: [usize; 2],
        tile_resolution: [usize; 2],
        cameras: Vec<CameraSettings>,
    ) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);
        debug_assert!(tile_resolution[0] > 0);
        debug_assert!(tile_resolution[1] > 0);
        debug_assert!(resolution[0] % tile_resolution[0] == 0);
        debug_assert!(resolution[1] % tile_resolution[1] == 0);
        debug_assert!(!cameras.is_empty());

        Self {
            resolution,
            tile_resolution,
            cameras,
        }
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.tile_resolution[0] > 0
            && self.tile_resolution[1] > 0
            && self.resolution[0] % self.tile_resolution[0] == 0
            && self.resolution[1] % self.tile_resolution[1] == 0
            && !self.cameras.is_empty()
    }

    /// Calculate the number of tiles in each dimension.
    pub fn num_tiles(&self) -> [usize; 2] {
        [
            self.resolution[0] / self.tile_resolution[0],
            self.resolution[1] / self.tile_resolution[1],
        ]
    }

    /// Calculate the total number of tiles.
    pub fn total_num_tiles(&self) -> usize {
        (self.resolution[0] / self.tile_resolution[0])
            * (self.resolution[1] / self.tile_resolution[1])
    }

    /// Get the resolution of each tile.
    pub fn tile_resolution(&self) -> [usize; 2] {
        self.tile_resolution
    }

    /// Calculate the total number of pixels per tile.
    pub fn total_num_tile_pixels(&self) -> usize {
        self.tile_resolution[0] * self.tile_resolution[1]
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "valid:                         {}", self.is_valid())?;

        writeln!(
            f,
            "resolution: {:>16} = {} pixels",
            format!("[{}, {}]", self.resolution[0], self.resolution[1]),
            self.resolution[0] * self.resolution[1]
        )?;

        writeln!(
            f,
            "tile resolution: {:>11} = {} pixels",
            format!("[{}, {}]", self.tile_resolution[0], self.tile_resolution[1]),
            self.tile_resolution[0] * self.tile_resolution[1]
        )?;

        let [num_x_tiles, num_y_tiles] = self.num_tiles();
        write!(
            f,
            "number of tiles: {:>11} = {} tiles",
            format!("[{}, {}]", num_x_tiles, num_y_tiles),
            num_x_tiles * num_y_tiles
        )?;

        for camera in &self.cameras {
            writeln!(f)?;
            writeln!(f, "camera: {}", camera.name)?;
            writeln!(
                f,
                "    position:  {:>16} = {}",
                format!(
                    "[{}, {}, {}]",
                    camera.position[0], camera.position[1], camera.position[2]
                ),
                nalgebra::Vector3::from_row_slice(&camera.position)
            )?;
            writeln!(
                f,
                "    target:    {:>16}",
                format!(
                    "[{}, {}, {}]",
                    camera.target[0], camera.target[1], camera.target[2]
                ),
            )?;
            writeln!(f, "    fov:       {} degrees", camera.field_of_view)?;
        }

        Ok(())
    }
}
