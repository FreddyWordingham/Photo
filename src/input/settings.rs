use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    fs::{read_to_string, write},
    path::Path,
    process::exit,
};

use crate::input::{CameraBuilder, LightingBuilder, SceneBuilder};

/// Runtime rendering settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Whether to print the tiles to the terminal as they are rendered.
    print_tiles_to_terminal: bool,
    /// Scene settings.
    scene: SceneBuilder,
    /// Lighting settings.
    lighting: LightingBuilder,
    /// List of cameras to render from.
    cameras: HashMap<String, CameraBuilder>,
}

impl Settings {
    /// Construct a new instance.
    pub fn new(
        print_tiles_to_terminal: bool,
        scene: SceneBuilder,
        lighting: LightingBuilder,
        cameras: HashMap<String, CameraBuilder>,
    ) -> Self {
        let settings = Self {
            print_tiles_to_terminal,
            scene,
            lighting,
            cameras,
        };

        debug_assert!(settings.is_valid());

        settings
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
        if !self.scene.is_valid() {
            println!("INVALID! Invalid scene");
            return false;
        }

        if self.cameras.is_empty() {
            println!("INVALID! No cameras");
            return false;
        }

        if !self.cameras.values().all(|c| c.is_valid()) {
            println!("INVALID! Invalid camera");
            return false;
        }

        true
    }

    /// Get whether to print the tiles to the terminal as they are rendered.
    pub fn print_tiles_to_terminal(&self) -> bool {
        self.print_tiles_to_terminal
    }

    /// Get the dictionary of cameras.
    pub fn cameras(&self) -> &HashMap<String, CameraBuilder> {
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
