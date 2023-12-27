//! Builder module.

#![allow(clippy::module_name_repetitions)]

mod camera_builder;
mod entity_builder;
mod light_builder;
mod material_builder;
mod settings_builder;
mod spectrum_builder;

pub use camera_builder::CameraBuilder;
pub use entity_builder::EntityBuilder;
pub use light_builder::LightBuilder;
pub use material_builder::MaterialBuilder;
pub use settings_builder::SettingsBuilder;
pub use spectrum_builder::SpectrumBuilder;
