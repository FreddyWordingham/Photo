use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use ndarray::{s, Array2};
use png::{ColorType, Decoder, Encoder};

use crate::image::Image;
use crate::image_error::ImageError;

impl Image for Array2<u8> {
    fn width(&self) -> u32 {
        self.ncols() as u32
    }

    fn height(&self) -> u32 {
        self.nrows() as u32
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let width = self.width();
        let height = self.height();

        debug_assert!(width > 0);
        debug_assert!(height > 0);

        // Create parent directories if they don't exist.
        if let Some(parent) = path.as_ref().parent() {
            create_dir_all(parent)?;
        }

        // Create the file and writer.
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        // Flip the image vertically.
        let flipped = self.slice(s![..;-1, ..]);

        // Convert to the correct type.
        let data: Vec<u8> = flipped.iter().cloned().collect();

        // Write the image data.
        writer.write_image_data(&data)?;
        Ok(())
    }

    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        // Load the file.
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

        // For 8-bit grayscale images, number of channels is 1 so total bytes = width * height.
        let channels = 1;
        let total_bytes = width * height * channels;
        let data: Vec<u8> = buf[..total_bytes].to_vec();

        // Create the image.
        let image = Array2::from_shape_vec((height, width), data)
            .map_err(|err| ImageError::ShapeError(err.to_string()))?;

        // Flip the image vertically.
        Ok(image.slice(s![..;-1, ..]).to_owned())
    }
}
