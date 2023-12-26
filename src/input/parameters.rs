//! Parameters builder structure.

use serde::{Deserialize, Serialize};

use crate::input::{
    CameraBuilder, EntityBuilder, MaterialBuilder, SettingsBuilder, SpectrumBuilder,
};

/// Input parameters object.
#[derive(Deserialize, Serialize)]
pub struct Parameters {
    /// Camera builder.
    pub camera: CameraBuilder,
    /// Entity builders.
    pub entities: Vec<EntityBuilder>,
    /// Material builders.
    pub materials: Vec<MaterialBuilder>,
    /// Settings builder.
    pub settings: SettingsBuilder,
    /// Spectrum builders.
    pub spectra: Vec<SpectrumBuilder>,
}
