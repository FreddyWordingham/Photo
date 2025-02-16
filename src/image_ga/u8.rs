use ndarray::{s, Array3};
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageGA};

impl ImageGA<u8> {
    /// Saves the grayscale + alpha image to the specified path in PNG format.
    ///
    /// # Errors
    ///
    /// Returns an `ImageError` with additional context if any IO or encoding error occurs.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let width = self.width() as u32;
        let height = self.height() as u32;
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        // Create parent directories with error context.
        if let Some(parent) = path.as_ref().parent() {
            create_dir_all(parent).map_err(|err| {
                ImageError::from_message(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    err
                ))
            })?;
        }

        // Create file, writer, and encoder.
        let file = File::create(&path).map_err(|err| {
            ImageError::from_message(format!(
                "Failed to create file {}: {}",
                path.as_ref().display(),
                err
            ))
        })?;
        let writer = BufWriter::new(file);
        let mut encoder = Encoder::new(writer, width, height);
        // Use GrayscaleAlpha for images with an alpha channel.
        encoder.set_color(ColorType::GrayscaleAlpha);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        // Flip the image vertically before saving.
        let flipped = self.data.slice(s![..;-1, .., ..]);
        let data: Vec<u8> = flipped.iter().cloned().collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads a PNG grayscale + alpha image from the specified path.
    ///
    /// # Errors
    ///
    /// Returns an `ImageError` with context if loading fails, or if the image has an unsupported format.
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

        // Check for GrayscaleAlpha format.
        if info.color_type != ColorType::GrayscaleAlpha || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 2;
        let total_bytes = width * height * channels;
        let data_vec: Vec<u8> = buffer[..total_bytes].to_vec();

        // Convert the flat vector into a 3D array with shape (height, width, 2).
        let image_array =
            Array3::from_shape_vec((height, width, channels), data_vec).map_err(|err| {
                ImageError::from_message(format!("Failed to create image array: {}", err))
            })?;

        // Flip vertically to match the expected orientation.
        let data = image_array.slice(s![..;-1, .., ..]).to_owned();
        Ok(Self { data })
    }
}

impl Display for ImageGA<u8> {
    /// Displays the image in the terminal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for pixel in row.outer_iter() {
                let value = pixel[0];
                let alpha = pixel[1];
                write!(f, "\x1b[48;2;{0};{0};{0};{1}m  \x1b[0m", value, alpha)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
