use std::{fs::File, io::BufWriter, path::Path};

use ndarray::Array2;
use num_traits::{Float, FromPrimitive};
use png::{ColorType, Decoder, Encoder};

use crate::{image::Image, image_error::ImageError};

impl<T> Image for Array2<T>
where
    T: Float + FromPrimitive,
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        if !self.iter().all(|&x| x >= T::zero() && x <= T::one()) {
            return Err(ImageError::PixelOutOfRange);
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let width = self.width();
        let height = self.height();

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data: Vec<u8> = self
            .outer_iter()
            .rev()
            .try_fold(Vec::new(), |mut acc, row| {
                for &x in row.iter() {
                    let value = (x * T::from(255.0).ok_or(ImageError::ConversionError)?)
                        .to_u8()
                        .ok_or(ImageError::ConversionError)?;
                    acc.push(value);
                }
                Ok::<Vec<u8>, ImageError>(acc)
            })?;

        writer.write_image_data(&data)?;

        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError>
    where
        Self: Sized,
    {
        let file = File::open(path)?;
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        if info.color_type != ColorType::Grayscale || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;

        let data: Vec<T> = buf[..info.buffer_size()]
            .iter()
            .map(|&x| {
                let value = T::from_u8(x).ok_or(ImageError::ConversionError)?;
                Ok(value / T::from_u8(255).unwrap())
            })
            .collect::<Result<Vec<T>, ImageError>>()?;

        Array2::from_shape_vec((height, width), data)
            .map_err(|e| ImageError::ShapeError(e.to_string()))
    }

    fn width(&self) -> u32 {
        self.ncols() as u32
    }

    fn height(&self) -> u32 {
        self.nrows() as u32
    }
}
