use ndarray::Array3;
use num_traits::NumCast;
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageGA, NormFloat};

impl<T: NormFloat> ImageGA<T> {
    /// Save the image in grayscale-alpha PNG format.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let width = self.width() as u32;
        let height = self.height() as u32;
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        if let Some(parent) = path.as_ref().parent() {
            create_dir_all(parent).map_err(|err| {
                ImageError::from_message(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    err
                ))
            })?;
        }

        let file = File::create(&path).map_err(|err| {
            ImageError::from_message(format!(
                "Failed to create file {}: {}",
                path.as_ref().display(),
                err
            ))
        })?;
        let writer = BufWriter::new(file);
        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::GrayscaleAlpha);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        let data: Vec<u8> = self.data.iter().map(|&v| v.to_u8()).collect();
        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Load a grayscale-alpha PNG image and converts it to normalized values.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let file = File::open(&path).map_err(|err| {
            ImageError::from_message(format!(
                "Failed to open file {}: {}",
                path.as_ref().display(),
                err
            ))
        })?;
        let decoder = Decoder::new(file);
        let mut reader = decoder
            .read_info()
            .map_err(|err| ImageError::from_message(format!("Failed to read PNG info: {}", err)))?;
        let mut buffer = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buffer).map_err(|err| {
            ImageError::from_message(format!("Failed to decode PNG frame: {}", err))
        })?;
        if info.color_type != ColorType::GrayscaleAlpha || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 2;
        let total_bytes = width * height * channels;
        let data = buffer[..total_bytes].to_vec();

        let image = Array3::from_shape_vec((height, width, channels), data).map_err(|err| {
            ImageError::from_message(format!("Failed to create image array: {}", err))
        })?;
        let divisor = T::from(255).unwrap();
        let data = image.map(|&v| T::from(v).unwrap() / divisor);
        Ok(Self { data })
    }
}

impl<T: NormFloat> Display for ImageGA<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for pixel in row.outer_iter() {
                let value = (pixel[0].clamp(T::zero(), T::one()) * T::from(255).unwrap()).round();
                let alpha = (pixel[1].clamp(T::zero(), T::one()) * T::from(255).unwrap()).round();
                let value_u8 = <u8 as NumCast>::from(value).unwrap();
                let alpha_u8 = <u8 as NumCast>::from(alpha).unwrap();
                write!(f, "\x1b[48;2;{0};{0};{0};{1}m  \x1b[0m", value_u8, alpha_u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
