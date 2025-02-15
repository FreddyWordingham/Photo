use ndarray::{arr1, s, stack, Array2, Array3, Axis};
use png::{ColorType, Decoder, Encoder};
use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageGA};

impl ImageGA<u8> {
    /// Creates a new ImageGA from the provided data.
    pub fn new(data: Array3<u8>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 2);
        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let mut data = Array3::zeros((height, width, 2));
        data.slice_mut(s![.., .., 1]).fill(255);
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(width: usize, height: usize, value: [u8; 2]) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let mut data = Array3::zeros((height, width, 2));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        Self { data }
    }

    /// Creates an ImageGA from two grayscale layers.
    pub fn from_layers(layers: [Array2<u8>; 2]) -> Self {
        // Ensure layers have matching dimensions.
        debug_assert!(layers.iter().all(|layer| layer.ncols() > 0));
        debug_assert!(layers.iter().all(|layer| layer.nrows() > 0));
        debug_assert!(layers.iter().all(|layer| layer.dim() == layers[0].dim()));
        let data =
            stack(Axis(2), &[layers[0].view(), layers[1].view()]).expect("Failed to stack layers");
        Self { data }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Get the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> u8 {
        debug_assert!(component < 2);
        self.data[[coords[1], coords[0], component]]
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: u8) {
        debug_assert!(component < 1);
        self.data[[coords[1], coords[0], component]] = value;
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [u8; 2] {
        let pixel_slice = self.data.slice(s![coords[1], coords[0], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice is not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [u8; 2]) {
        let mut view = self.data.slice_mut(s![coords[1], coords[0], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Get a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<u8> {
        debug_assert!(component < 2);
        self.data.slice(s![.., .., component]).to_owned()
    }

    /// Transposes the image.
    pub fn transpose(&mut self) {
        self.data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
    }

    /// Flips the image vertically.
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    /// Flips the image horizontally.
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Rotates the image 90 degrees clockwise.
    /// This is achieved by transposing the image and then flipping vertically.
    pub fn rotate_clockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(1));
        self.data = new_data;
    }

    /// Rotates the image 90 degrees anticlockwise.
    /// This is achieved by transposing the image and then flipping horizontally.
    pub fn rotate_anticlockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(0));
        self.data = new_data;
    }

    /// Rotates the image 180 degrees by flipping both axes.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
    }

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

impl std::fmt::Display for ImageGA<u8> {
    /// Displays the image in the terminal.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
