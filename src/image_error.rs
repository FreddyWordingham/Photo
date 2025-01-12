use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("Failed to create or write to file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to encode PNG: {0}")]
    EncodeError(#[from] png::EncodingError),
    #[error("Failed to decode PNG: {0}")]
    DecodeError(#[from] png::DecodingError),
    #[error("Unsupported color type. Only RGB or RGBA images are supported.")]
    UnsupportedColorType,
    #[error("Invalid image shape: must have 3 or 4 channels.")]
    InvalidImageShape,
    #[error("Pixel value out of range. Values must be between 0 and 1.")]
    PixelOutOfRange,
    #[error("Conversion error while scaling pixel values.")]
    ConversionError,
    #[error("Shape mismatch: {0}")]
    ShapeError(String),
}
