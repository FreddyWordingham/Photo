use ndarray::{arr1, s, stack, Array2, Array3, Axis};
use png::{ColorType, Decoder, Encoder};
use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageRGBA};

impl ImageRGBA<f32> {
    /// Creates a new ImageRGBA from the provided data.
    pub fn new(data: Array3<f32>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 4);
        Self { data }
    }

    /// Creates an empty image with the given dimensions.
    /// The RGB channels are zeroed and the alpha channel is set to 1.0.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0 && height > 0);
        let mut data = Array3::zeros((height, width, 4));
        data.slice_mut(s![.., .., 3]).fill(1.0);
        Self { data }
    }

    /// Creates an image filled with a constant RGBA value.
    pub fn filled(width: usize, height: usize, value: [f32; 4]) -> Self {
        debug_assert!(width > 0 && height > 0);
        let mut data = Array3::zeros((height, width, 4));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        data.slice_mut(s![.., .., 2]).fill(value[2]);
        data.slice_mut(s![.., .., 3]).fill(value[3]);
        Self { data }
    }

    /// Creates an ImageRGBA from four layers (red, green, blue, alpha).
    pub fn from_layers(layers: [Array2<f32>; 4]) -> Self {
        debug_assert!(layers.iter().all(|layer| layer.ncols() > 0));
        debug_assert!(layers.iter().all(|layer| layer.nrows() > 0));
        debug_assert!(layers.iter().all(|layer| layer.dim() == layers[0].dim()));
        let data = stack(
            Axis(2),
            &[
                layers[0].view(),
                layers[1].view(),
                layers[2].view(),
                layers[3].view(),
            ],
        )
        .expect("Failed to stack layers");
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
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> f32 {
        debug_assert!(component < 4);
        self.data[[coords[1], coords[0], component]]
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: f32) {
        debug_assert!(component < 4);
        self.data[[coords[1], coords[0], component]] = value;
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [f32; 4] {
        let pixel_slice = self.data.slice(s![coords[1], coords[0], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice is not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [f32; 4]) {
        let mut view = self.data.slice_mut(s![coords[1], coords[0], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Get a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<f32> {
        debug_assert!(component < 4);
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
    pub fn rotate_clockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(1));
        self.data = new_data;
    }

    /// Rotates the image 90 degrees anticlockwise.
    pub fn rotate_anticlockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(0));
        self.data = new_data;
    }

    /// Rotates the image 180 degrees.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
    }

    /// Saves the RGBA image to the specified path in PNG format.
    /// The internal float data ([0.0, 1.0]) is clamped and converted to u8.
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
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        // Convert float data [0.0, 1.0] to u8 and flip vertically.
        let flipped = self.data.slice(s![..;-1, .., ..]);
        let data: Vec<u8> = flipped
            .iter()
            .map(|&v| ((v.clamp(0.0, 1.0)) * 255.0).round() as u8)
            .collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads an RGBA PNG image and converts it to float representation.
    /// The resulting values are normalized to the range [0.0, 1.0].
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
        let data_vec: Vec<u8> = buffer[..total_bytes].to_vec();

        let image_array =
            Array3::from_shape_vec((height, width, channels), data_vec).map_err(|err| {
                ImageError::from_message(format!("Failed to create image array: {}", err))
            })?;

        // Flip vertically and convert u8 to f32.
        let data = image_array
            .slice(s![..;-1, .., ..])
            .map(|&v| (v as f32) / 255.0)
            .to_owned();
        Ok(Self { data })
    }
}

impl std::fmt::Display for ImageRGBA<f32> {
    /// Displays the image in the terminal.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for pixel in row.outer_iter() {
                let r = (pixel[0].clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel[1].clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel[2].clamp(0.0, 1.0) * 255.0) as u8;
                let a = (pixel[3].clamp(0.0, 1.0) * 255.0) as u8;
                write!(f, "\x1b[48;2;{r};{g};{b}m{a:3}\x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
