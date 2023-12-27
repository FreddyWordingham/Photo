//! Error module.

#![allow(clippy::module_name_repetitions)]

mod build_error;
mod validation_error;

pub use build_error::BuildError;
pub use validation_error::ValidationError;
