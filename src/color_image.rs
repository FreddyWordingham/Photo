use std::{fs::File, io::BufWriter, path::Path};

use ndarray::{Array3, Axis};
use num_traits::{Float, FromPrimitive};
use png::{ColorType, Decoder, Encoder};

use crate::{image::Image, image_error::ImageError};

impl<T> Image for Array3<T>
where
    T: Float + FromPrimitive,
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        // Accept 2 (grayscale with alpha), 3 (RGB) or 4 (RGBA) channels.
        let channels = self.shape()[2];
        if !(channels == 2 || channels == 3 || channels == 4) {
            return Err(ImageError::InvalidImageShape);
        }

        if !self.iter().all(|&x| x >= T::zero() && x <= T::one()) {
            return Err(ImageError::PixelOutOfRange);
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let (height, width) = (self.shape()[0] as u32, self.shape()[1] as u32);

        let color_type = match channels {
            2 => ColorType::GrayscaleAlpha,
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => unreachable!(),
        };

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(color_type);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = self
            .axis_iter(Axis(0))
            .rev()
            .map(|row| {
                row.iter()
                    .map(|&x| {
                        (x * T::from(255.0).ok_or(ImageError::ConversionError)?)
                            .to_u8()
                            .ok_or(ImageError::ConversionError)
                    })
                    .collect::<Result<Vec<u8>, ImageError>>()
            })
            .collect::<Result<Vec<Vec<u8>>, ImageError>>()?
            .into_iter()
            .flatten()
            .collect();

        writer.write_image_data(&data)?;

        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let file = File::open(path)?;
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let channels = match info.color_type {
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
            ColorType::GrayscaleAlpha => 2,
            _ => return Err(ImageError::UnsupportedColorType),
        };

        let width = info.width as usize;
        let height = info.height as usize;

        let data: Vec<T> = buf[..info.buffer_size()]
            .iter()
            .map(|&x| {
                let value = T::from_u8(x).ok_or(ImageError::ConversionError)?;
                Ok(value / T::from_u8(255).unwrap())
            })
            .collect::<Result<Vec<T>, ImageError>>()?;

        let mut array = Array3::<T>::zeros((height, width, channels));
        for (idx, val) in data.into_iter().enumerate() {
            let row = idx / (width * channels);
            let col = (idx % (width * channels)) / channels;
            let ch = (idx % (width * channels)) % channels;
            let flipped_row = height - row - 1;
            array[[flipped_row, col, ch]] = val;
        }

        Ok(array)
    }

    fn width(&self) -> u32 {
        self.shape()[1] as u32
    }

    fn height(&self) -> u32 {
        self.shape()[0] as u32
    }
}
