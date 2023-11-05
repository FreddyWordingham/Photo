use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use crate::input::CameraSettings;

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Whether to print the tiles to the terminal as they are rendered.
    print_tiles_to_terminal: bool,
    /// List of cameras to render from.
    cameras: HashMap<String, CameraSettings>,
}

impl Settings {
    /// Construct a new Settings object.
    pub fn new(print_tiles_to_terminal: bool, cameras: HashMap<String, CameraSettings>) -> Self {
        debug_assert!(!cameras.is_empty());
        debug_assert!(cameras.values().all(|c| c.is_valid()));

        Self {
            print_tiles_to_terminal,
            cameras,
        }
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        !self.cameras.is_empty() && self.cameras.values().all(|c| c.is_valid())
    }

    /// Get whether to print the tiles to the terminal as they are rendered.
    pub fn print_tiles_to_terminal(&self) -> bool {
        self.print_tiles_to_terminal
    }

    /// Get the dictionary of cameras.
    pub fn cameras(&self) -> &HashMap<String, CameraSettings> {
        &self.cameras
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

        write!(
            f,
            "number of cameras:     {:>9} cameras",
            self.cameras.len()
        )?;

        Ok(())
    }
}
