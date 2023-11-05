use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Runtime rendering settings.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Settings {
    /// The resolution of the image in pixels.
    pub resolution: [usize; 2],
    /// The resolution of each tile in pixels.
    pub tile_resolution: [usize; 2],
    /// Display the tiles as they are rendered.
    pub display_tiles: bool,
}

impl Settings {
    /// Construct a new Settings object.
    pub const fn new(
        resolution: [usize; 2],
        tile_resolution: [usize; 2],
        display_tiles: bool,
    ) -> Self {
        debug_assert!(resolution[0] > 0);
        debug_assert!(resolution[1] > 0);
        debug_assert!(tile_resolution[0] > 0);
        debug_assert!(tile_resolution[1] > 0);
        debug_assert!(resolution[0] % tile_resolution[0] == 0);
        debug_assert!(resolution[1] % tile_resolution[1] == 0);

        Self {
            resolution,
            tile_resolution,
            display_tiles,
        }
    }

    /// Check that the current combination of values is valid.
    pub fn is_valid(&self) -> bool {
        self.resolution[0] > 0
            && self.resolution[1] > 0
            && self.tile_resolution[0] > 0
            && self.tile_resolution[1] > 0
            && self.resolution[0] % self.tile_resolution[0] == 0
            && self.resolution[1] % self.tile_resolution[1] == 0
    }

    /// Calculate the number of tiles in each dimension.
    pub fn num_tiles(&self) -> [usize; 2] {
        debug_assert!(self.is_valid());

        [
            self.resolution[0] / self.tile_resolution[0],
            self.resolution[1] / self.tile_resolution[1],
        ]
    }

    /// Calculate the total number of tiles.
    pub fn total_num_tiles(&self) -> usize {
        debug_assert!(self.is_valid());

        (self.resolution[0] / self.tile_resolution[0])
            * (self.resolution[1] / self.tile_resolution[1])
    }

    /// Get the resolution of each tile.
    pub fn tile_resolution(&self) -> [usize; 2] {
        debug_assert!(self.is_valid());

        self.tile_resolution
    }

    /// Calculate the total number of pixels per tile.
    pub fn total_num_tile_pixels(&self) -> usize {
        debug_assert!(self.is_valid());

        self.tile_resolution[0] * self.tile_resolution[1]
    }
}

impl Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "valid:                         {}\n", self.is_valid())?;

        write!(
            f,
            "resolution: {:>16} = {}\n",
            format!("[{}, {}]", self.resolution[0], self.resolution[1]),
            self.resolution[0] * self.resolution[1]
        )?;

        write!(
            f,
            "tile resolution: {:>11} = {}\n",
            format!("[{}, {}]", self.tile_resolution[0], self.tile_resolution[1]),
            self.tile_resolution[0] * self.tile_resolution[1]
        )?;

        let [num_x_tiles, num_y_tiles] = self.num_tiles();
        write!(
            f,
            "number of tiles: {:>11} = {}",
            format!("[{}, {}]", num_x_tiles, num_y_tiles),
            num_x_tiles * num_y_tiles
        )
    }
}
