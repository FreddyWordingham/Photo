use std::{fs::File, io::BufWriter, path::Path};

use ndarray::{s, Array2, Array3};
use num_traits::{Float, FromPrimitive};
use png::{ColorType, Decoder, Encoder};

use crate::image_error::ImageError;

pub trait Image {
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError>;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError>
    where
        Self: Sized;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

fn to_u8<T: Float + FromPrimitive>(x: T) -> Result<u8, ImageError> {
    let max = T::from(255.0).ok_or(ImageError::ConversionError)?;
    (x * max).to_u8().ok_or(ImageError::ConversionError)
}

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
        let width = self.ncols() as u32;
        let height = self.nrows() as u32;

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        // Flip the image vertically.
        let flipped = self.slice(s![..;-1, ..]);
        let data: Vec<u8> = flipped
            .iter()
            .map(|&x| to_u8(x))
            .collect::<Result<_, _>>()?;

        writer.write_image_data(&data)?;
        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
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
        // For 8-bit grayscale, total bytes = width * height.
        let total_bytes = width * height;
        let scale = T::from_u8(255).ok_or(ImageError::ConversionError)?;
        let data: Vec<T> = buf[..total_bytes]
            .iter()
            .map(|&x| {
                T::from_u8(x)
                    .ok_or(ImageError::ConversionError)
                    .map(|v| v / scale)
            })
            .collect::<Result<_, _>>()?;

        let image = Array2::from_shape_vec((height, width), data)
            .map_err(|e| ImageError::ShapeError(e.to_string()))?;
        // Flip the image vertically to restore the original orientation.
        Ok(image.slice(s![..;-1, ..]).to_owned())
    }
    fn width(&self) -> u32 {
        self.ncols() as u32
    }

    fn height(&self) -> u32 {
        self.nrows() as u32
    }
}

impl<T> Image for Array3<T>
where
    T: Float + FromPrimitive,
{
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let channels = self.shape()[2];
        if !(channels == 2 || channels == 3 || channels == 4) {
            return Err(ImageError::InvalidImageShape);
        }
        if !self.iter().all(|&x| x >= T::zero() && x <= T::one()) {
            return Err(ImageError::PixelOutOfRange);
        }

        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let width = self.shape()[1] as u32;
        let height = self.shape()[0] as u32;
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

        // Flip the image vertically.
        let flipped = self.slice(s![..;-1, .., ..]);
        // For 8-bit images, each channel is 1 byte.
        let data: Vec<u8> = flipped
            .iter()
            .map(|&x| to_u8(x))
            .collect::<Result<_, _>>()?;

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
        // For 8-bit images, total bytes = width * height * channels.
        let total_bytes = width * height * channels;
        let scale = T::from_u8(255).ok_or(ImageError::ConversionError)?;
        let data: Vec<T> = buf[..total_bytes]
            .iter()
            .map(|&x| {
                T::from_u8(x)
                    .ok_or(ImageError::ConversionError)
                    .map(|v| v / scale)
            })
            .collect::<Result<_, _>>()?;

        let array = Array3::from_shape_vec((height, width, channels), data)
            .map_err(|e| ImageError::ShapeError(e.to_string()))?;
        // Flip the image vertically to correct the orientation.
        Ok(array.slice(s![..;-1, .., ..]).to_owned())
    }

    fn width(&self) -> u32 {
        self.shape()[1] as u32
    }

    fn height(&self) -> u32 {
        self.shape()[0] as u32
    }
}
