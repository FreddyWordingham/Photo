//! Build error enumeration.

use core::fmt::{Display, Formatter, Result};
use std::error::Error;

/// Build error.
#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError {
    IdentifierNotFound(String),
    SpectrumNotFound(String),
    MeshNotFound(String),
    MaterialNotFound(String),
}

impl Display for BuildError {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            Self::IdentifierNotFound(identifier) => {
                write!(
                    formatter,
                    "Build error: identifier not found: {identifier}!"
                )
            }
            Self::SpectrumNotFound(spectrum_id) => {
                write!(formatter, "Build error: spectrum not found: {spectrum_id}!")
            }
            Self::MeshNotFound(mesh_id) => {
                write!(formatter, "Build error: mesh not found: {mesh_id}!")
            }
            Self::MaterialNotFound(material_id) => {
                write!(formatter, "Build error: material not found: {material_id}!")
            }
        }
    }
}

impl Error for BuildError {}
