use ndarray::Array2;
use palette::LinSrgba;
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{Image, ImageError};

impl Image<LinSrgba> {
    /// Get the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> f32 {
        debug_assert!(component < 4);
        let colour = self.data[[coords[1], coords[0]]];
        match component {
            0 => colour.red,
            1 => colour.green,
            2 => colour.blue,
            3 => colour.alpha,
            _ => unreachable!(),
        }
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: f32) {
        debug_assert!(component < 4);
        let mut colour = self.data[[coords[1], coords[0]]];
        match component {
            0 => colour.red = value,
            1 => colour.green = value,
            2 => colour.blue = value,
            3 => colour.alpha = value,
            _ => unreachable!(),
        }
    }

    /// Save the image in RGBA PNG format.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let height = self.data.nrows();
        let width = self.data.ncols();
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
        let mut encoder = Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut png_writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        let mut data = Vec::with_capacity(width * height * 4);
        for row in self.data.outer_iter() {
            for color in row.iter() {
                let r = (color.red.clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (color.green.clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (color.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
                let a = (color.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
                data.extend_from_slice(&[r, g, b, a]);
            }
        }
        png_writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Load a RGBA PNG image and converts it to normalized values.
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
        if info.color_type != ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 4;
        let total_bytes = width * height * channels;
        let data_vec = buffer[..total_bytes].to_vec();

        let data = Array2::from_shape_fn((height, width), |(y, x)| {
            let i = (y * width + x) * channels;
            let r = data_vec[i] as f32 / 255.0;
            let g = data_vec[i + 1] as f32 / 255.0;
            let b = data_vec[i + 2] as f32 / 255.0;
            let a = data_vec[i + 3] as f32 / 255.0;
            LinSrgba::new(r, g, b, a)
        });
        Ok(Self { data })
    }
}

impl Display for Image<LinSrgba> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter() {
            for pixel in row.iter() {
                let red = (pixel.red.clamp(0.0, 1.0) * 255.0) as u8;
                let green = (pixel.green.clamp(0.0, 1.0) * 255.0) as u8;
                let blue = (pixel.blue.clamp(0.0, 1.0) * 255.0) as u8;
                let alpha = (pixel.alpha.clamp(0.0, 1.0) * 255.0) as u8;
                write!(f, "\x1b[48;2;{red};{green};{blue};{alpha}m  \x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
