//! Error module.

#![allow(clippy::module_name_repetitions)]

mod build_error;
mod parse_error;
mod save_error;
mod validation_error;

pub use build_error::BuildError;
pub use parse_error::ParseError;
pub use save_error::SaveError;
pub use validation_error::ValidationError;
