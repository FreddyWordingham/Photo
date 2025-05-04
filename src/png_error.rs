use std::{
    error::Error,
    fmt::{self, Formatter, Result as FmtResult},
    io::Error as IoError,
};

/// Errors that can occur during PNG image operations.
#[derive(Debug)]
pub enum PngError {
    IoError(IoError),
    PngError(png::DecodingError),
    EncodingError(png::EncodingError),
    UnsupportedColourType(png::ColorType),
    UnsupportedBitDepth(png::BitDepth),
    InvalidChannelCount,
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PngError::IoError(err) => write!(f, "IO error: {}", err),
            PngError::PngError(err) => write!(f, "PNG decoding error: {}", err),
            PngError::EncodingError(err) => write!(f, "PNG encoding error: {}", err),
            PngError::UnsupportedColourType(color_type) => write!(f, "Unsupported color type: {:?}", color_type),
            PngError::UnsupportedBitDepth(bit_depth) => write!(f, "Unsupported bit depth: {:?}", bit_depth),
            PngError::InvalidChannelCount => write!(f, "Invalid channel count for colour type"),
        }
    }
}

impl Error for PngError {}

impl From<IoError> for PngError {
    fn from(err: IoError) -> Self {
        PngError::IoError(err)
    }
}

impl From<png::DecodingError> for PngError {
    fn from(err: png::DecodingError) -> Self {
        PngError::PngError(err)
    }
}

impl From<png::EncodingError> for PngError {
    fn from(err: png::EncodingError) -> Self {
        PngError::EncodingError(err)
    }
}
