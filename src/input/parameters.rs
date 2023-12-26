//! Parameters builder structure.

use serde::{Deserialize, Serialize};

use crate::input::{
    CameraBuilder, EntityBuilder, MaterialBuilder, SettingsBuilder, SpectrumBuilder,
};

/// Input parameters object.
#[derive(Deserialize, Serialize)]
pub struct Parameters {
    /// Camera builder.
    camera: CameraBuilder,
    /// Entity builders.
    entities: Vec<EntityBuilder>,
    /// Material builders.
    materials: Vec<MaterialBuilder>,
    /// Settings builder.
    settings: SettingsBuilder,
    /// Spectrum builders.
    spectra: Vec<SpectrumBuilder>,
}
