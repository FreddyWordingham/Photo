use ndarray::{s, Array3};
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageRGB};

impl ImageRGB<u8> {
    /// Saves the RGB image to the specified path in PNG format.
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
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        // Flip vertically for correct orientation.
        let flipped = self.data.slice(s![..;-1, .., ..]);
        let data: Vec<u8> = flipped.iter().cloned().collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads an RGB PNG image from the specified path.
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

        if info.color_type != ColorType::Rgb || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 3;
        let total_bytes = width * height * channels;
        let data_vec: Vec<u8> = buffer[..total_bytes].to_vec();

        let image_array =
            Array3::from_shape_vec((height, width, channels), data_vec).map_err(|err| {
                ImageError::from_message(format!("Failed to create image array: {}", err))
            })?;

        // Flip vertically to match the expected orientation.
        let data = image_array.slice(s![..;-1, .., ..]).to_owned();
        Ok(Self { data })
    }
}

impl Display for ImageRGB<u8> {
    /// Displays the image in the terminal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for pixel in row.outer_iter() {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                write!(f, "\x1b[48;2;{r};{g};{b}m  \x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
