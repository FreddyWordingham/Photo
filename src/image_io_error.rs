//! ## `io` image file operations.
//!
//! This module provides functionality for reading and writing images to and from files.

use core::num::TryFromIntError;
use png::{BitDepth, ColorType, DecodingError, EncodingError};
use std::io::Error as IoError;
use thiserror::Error;

use crate::Channels;

/// Error that can occur when reading or writing images.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ImageIoError {
    /// Error from the PNG library.
    #[error("PNG error: {0}")]
    PngError(#[from] DecodingError),

    /// Error when encoding a PNG.
    #[error("PNG encoding error: {0}")]
    PngEncodingError(#[from] EncodingError),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] IoError),

    /// Error converting between number types.
    #[error("Conversion error: Unable to convert between number types")]
    ConversionError,

    /// Error converting integer types.
    #[error("Integer conversion error: {0}")]
    IntegerConversionError(#[from] TryFromIntError),

    /// Unsupported channel format.
    #[error("Unsupported channel format: {0:?}")]
    UnsupportedChannelFormat(Channels),

    /// Unsupported bit depth.
    #[error("Unsupported bit depth: {0:?}")]
    UnsupportedBitDepth(BitDepth),

    /// Unsupported color type.
    #[error("Unsupported color type: {0:?}")]
    UnsupportedColorType(ColorType),
}
