use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

use crate::input::CameraSettings;

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Whether to print the tiles to the terminal as they are rendered.
    print_tiles_to_terminal: bool,
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
        print_tiles_to_terminal: bool,
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
        debug_assert!(cameras.iter().all(|c| c.is_valid()));

        Self {
            print_tiles_to_terminal,
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
            && self.cameras.iter().all(|c| c.is_valid())
    }

    /// Get whether to print the tiles to the terminal as they are rendered.
    pub fn print_tiles_to_terminal(&self) -> bool {
        self.print_tiles_to_terminal
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

impl Display for Settings {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "valid:                         {}", self.is_valid())?;

        writeln!(
            f,
            "print tiles to terminal:       {}",
            self.print_tiles_to_terminal
        )?;

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
        writeln!(
            f,
            "number of tiles: {:>11} = {} tiles",
            format!("[{}, {}]", num_x_tiles, num_y_tiles),
            num_x_tiles * num_y_tiles
        )?;

        write!(
            f,
            "number of cameras:     {:>9} cameras",
            self.cameras.len()
        )?;

        Ok(())
    }
}
