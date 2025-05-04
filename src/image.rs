use chromatic::Colour;
use ndarray::Array2;
use num_traits::Float;
use png::ColorType;
use std::{
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::PngError;

/// Trait for image encoding/decoding operations on Array2<C> where C is a Colour.
pub trait Image<C, T, const N: usize>
where
    C: Colour<T, N> + Clone,
    T: Float + Send + Sync,
{
    /// Error type returned by read/write operations.
    type Error: std::error::Error;

    /// Read an image from a file path.
    fn load<P: AsRef<Path>>(path: P) -> Result<Array2<C>, Self::Error>;

    /// Write an image to a file path.
    fn save<P: AsRef<Path>>(image: &Array2<C>, path: P) -> Result<(), Self::Error>;

    /// Read an image from a reader.
    fn read<R: Read>(reader: R) -> Result<Array2<C>, Self::Error>;

    /// Write an image to a writer.
    fn write<W: Write>(image: &Array2<C>, writer: W) -> Result<(), Self::Error>;
}

impl<C, T, const N: usize> Image<C, T, N> for Array2<C>
where
    C: Colour<T, N> + Clone,
    T: Float + Send + Sync,
{
    type Error = PngError;

    fn load<P: AsRef<Path>>(path: P) -> Result<Array2<C>, Self::Error> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        Self::read(reader)
    }

    fn save<P: AsRef<Path>>(image: &Array2<C>, path: P) -> Result<(), Self::Error> {
        let file = std::fs::File::create(path)?;
        let writer = BufWriter::new(file);
        Self::write(image, writer)
    }

    fn read<R: Read>(reader: R) -> Result<Array2<C>, Self::Error> {
        let decoder = png::Decoder::new(reader);
        let mut reader = decoder.read_info()?;

        let info = reader.info();
        let width = info.width as usize;
        let height = info.height as usize;

        // Determine colour type based on `NUM_COMPONENTS`.
        let expected_channels = match N {
            1 => ColorType::Grayscale,
            2 => ColorType::GrayscaleAlpha,
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => return Err(PngError::InvalidChannelCount),
        };

        // Check that the PNG's colour type matches what we expect
        if !match_colour_types(info.color_type, expected_channels) {
            return Err(PngError::UnsupportedColourType(info.color_type));
        }

        // Check bit depth
        if info.bit_depth != png::BitDepth::Eight {
            return Err(PngError::UnsupportedBitDepth(info.bit_depth));
        }

        let bytes_per_pixel = info.color_type.samples() as usize;

        // Allocate the output buffer
        let mut buffer = vec![0; width * height * bytes_per_pixel];

        // Read image data
        reader.next_frame(&mut buffer)?;

        // Convert to Array2<C>
        let mut image = Array2::from_elem((height, width), C::from_bytes([0; N]));
        for y in 0..height {
            for x in 0..width {
                let idx = (y * width + x) * bytes_per_pixel;

                // Extract bytes for this pixel
                let mut pixel_bytes = [0u8; N];
                for i in 0..N {
                    pixel_bytes[i] = buffer[idx + i];
                }

                image[[y, x]] = C::from_bytes(pixel_bytes);
            }
        }

        Ok(image)
    }

    fn write<W: Write>(image: &Array2<C>, writer: W) -> Result<(), Self::Error> {
        let (height, width) = image.dim();

        // Determine colour type based on NUM_COMPONENTS
        let colour_type = match N {
            1 => ColorType::Grayscale,
            2 => ColorType::GrayscaleAlpha,
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => return Err(PngError::InvalidChannelCount),
        };

        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(colour_type);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header()?;

        // Convert Array2<C> to raw bytes
        let mut buffer = Vec::with_capacity(width * height * N);

        for y in 0..height {
            for x in 0..width {
                let pixel = image[[y, x]].clone();
                let pixel_bytes = pixel.to_bytes();

                for i in 0..N {
                    buffer.push(pixel_bytes[i]);
                }
            }
        }

        writer.write_image_data(&buffer)?;

        Ok(())
    }
}

/// Helper function to check if the colour types are compatible
fn match_colour_types(actual: ColorType, expected: ColorType) -> bool {
    // Exact match
    if actual == expected {
        return true;
    }

    // Handle special cases where we can convert between formats
    match (actual, expected) {
        // Can convert RGB to RGBA by adding alpha channel
        (ColorType::Rgb, ColorType::Rgba) => true,

        // Can convert Grayscale to GrayscaleAlpha by adding alpha
        (ColorType::Grayscale, ColorType::GrayscaleAlpha) => true,

        // Can ignore alpha when reading RGBA as RGB
        (ColorType::Rgba, ColorType::Rgb) => true,

        // Can ignore alpha when reading GrayscaleAlpha as Grayscale
        (ColorType::GrayscaleAlpha, ColorType::Grayscale) => true,

        // All other combinations are not compatible
        _ => false,
    }
}
