use ndarray::{s, Array3};
use num_traits::NumCast;
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageRGB, NormFloat};

impl<T: NormFloat> ImageRGB<T> {
    /// Saves the RGB image to the specified path in PNG format.
    /// Values are clamped to [0,1] and converted to u8.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let width = self.width() as u32;
        let height = self.height() as u32;
        debug_assert!(width > 0 && height > 0);

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

        // Flip vertically and convert normalized T to u8.
        let flipped = self.data.slice(s![..;-1, .., ..]);
        let data: Vec<u8> = flipped.iter().map(|&v| v.to_u8()).collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads an RGB PNG image and converts it to a normalized ImageRGB.
    /// The resulting values are normalized to the range [0,1].
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
        let data_vec = buffer[..total_bytes].to_vec();

        let image_array =
            Array3::from_shape_vec((height, width, channels), data_vec).map_err(|err| {
                ImageError::from_message(format!("Failed to create image array: {}", err))
            })?;

        // Flip vertically and convert u8 to normalized T.
        let divisor = T::from(255).unwrap();
        let data = image_array
            .slice(s![..;-1, .., ..])
            .map(|&v| T::from(v).unwrap() / divisor)
            .to_owned();
        Ok(Self { data })
    }
}

impl<T: NormFloat> Display for ImageRGB<T> {
    /// Displays the image in the terminal.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for pixel in row.outer_iter() {
                let r = pixel[0].max(T::zero()).min(T::one()) * T::from(255).unwrap();
                let g = pixel[1].max(T::zero()).min(T::one()) * T::from(255).unwrap();
                let b = pixel[2].max(T::zero()).min(T::one()) * T::from(255).unwrap();
                let r_u8 = <u8 as NumCast>::from(r.round()).unwrap();
                let g_u8 = <u8 as NumCast>::from(g.round()).unwrap();
                let b_u8 = <u8 as NumCast>::from(b.round()).unwrap();
                write!(f, "\x1b[48;2;{r_u8};{g_u8};{b_u8}m  \x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
