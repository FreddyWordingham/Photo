use chromatic::Colour;
use ndarray::Array2;
use num_traits::Float;
use png::{BitDepth, ColorType, Decoder, Encoder};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use crate::{Image, PngError};

impl<C, T, const N: usize> Image<C, T, N> for Array2<C>
where
    C: Colour<T, N> + Copy,
    T: Float + Send + Sync,
{
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, PngError> {
        let rd = BufReader::new(File::open(path)?);
        Self::read(rd)
    }

    fn read<R: Read>(reader: R) -> Result<Self, PngError> {
        let mut reader = Decoder::new(reader).read_info()?;
        let info = reader.info();
        let (w, h) = (info.width as usize, info.height as usize);

        // Check bit depth
        if info.bit_depth != BitDepth::Eight {
            return Err(PngError::UnsupportedBitDepth(info.bit_depth));
        }

        // Match expected color
        let expected = match N {
            1 => ColorType::Grayscale,
            2 => ColorType::GrayscaleAlpha,
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => return Err(PngError::InvalidChannelCount),
        };
        if !match_colour_types(info.color_type, expected) {
            return Err(PngError::UnsupportedColourType(info.color_type));
        }

        // Read frame
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;

        // One-liner conversion
        let pixels = buf
            .chunks_exact(N)
            .map(|chunk| {
                let mut arr = [0u8; N];
                arr.copy_from_slice(chunk);
                C::from_bytes(arr)
            })
            .collect::<Vec<_>>();

        Array2::from_shape_vec((h, w), pixels).map_err(|_| PngError::InvalidData)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), PngError> {
        let wr = BufWriter::new(File::create(path)?);
        Self::write(self, wr)
    }

    fn write<W: Write>(&self, mut writer: W) -> Result<(), PngError> {
        let (h, w) = self.dim();
        let colour = match N {
            1 => ColorType::Grayscale,
            2 => ColorType::GrayscaleAlpha,
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => return Err(PngError::InvalidChannelCount),
        };

        let mut enc = Encoder::new(&mut writer, w as u32, h as u32);
        enc.set_color(colour);
        enc.set_depth(BitDepth::Eight);
        let mut whdr = enc.write_header()?;

        // Flat-map + extend
        let mut bytes = Vec::with_capacity(w * h * N);
        bytes.extend(self.iter().flat_map(|px| px.to_bytes()));

        whdr.write_image_data(&bytes)?;
        Ok(())
    }
}

/// Helper function to check if the colour types are compatible.
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
