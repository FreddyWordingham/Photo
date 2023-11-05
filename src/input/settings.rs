use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    fs::{read_to_string, write},
    path::Path,
    process::exit,
};

use crate::input::{CameraSettings, LightingSettings, SceneSettings};

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Whether to print the tiles to the terminal as they are rendered.
    print_tiles_to_terminal: bool,
    /// Scene settings.
    scene: SceneSettings,
    /// Lighting settings.
    lighting: LightingSettings,
    /// List of cameras to render from.
    cameras: HashMap<String, CameraSettings>,
}

impl Settings {
    /// Construct a new Settings object.
    pub fn new(
        print_tiles_to_terminal: bool,
        scene: SceneSettings,
        lighting: LightingSettings,
        cameras: HashMap<String, CameraSettings>,
    ) -> Self {
        debug_assert!(scene.is_valid());
        debug_assert!(lighting.is_valid());
        debug_assert!(!cameras.is_empty());
        debug_assert!(cameras.values().all(|c| c.is_valid()));

        Self {
            print_tiles_to_terminal,
            scene,
            lighting,
            cameras,
        }
    }

    /// Load the settings from the given file.
    pub fn load(settings_filepath: &Path) -> Settings {
        let file_string = read_to_string(settings_filepath).expect("Unable to read settings file");

        let settings: Settings =
            serde_yaml::from_str(&file_string).expect("Unable to parse settings file");

        if !settings.is_valid() {
            println!("Invalid settings file: {}", settings_filepath.display());
            exit(1);
        }

        settings
    }

    /// Save the settings to the given file.
    pub fn save(&self, settings_filepath: &Path) {
        let file_string =
            serde_yaml::to_string(self).expect("Unable to serialise settings to string");

        write(settings_filepath, file_string).expect("Unable to write settings file");
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
