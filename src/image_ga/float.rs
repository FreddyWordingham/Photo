use ndarray::{arr1, s, stack, Array2, Array3, Axis};
use num_traits::{Float, NumCast};
use png::{ColorType, Decoder, Encoder};
use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageGA};

/// Helper trait to convert normalized float values ([0,1]) to u8.
pub trait NormFloat: Float + NumCast {
    fn to_u8(self) -> u8 {
        let clamped = self.max(Self::zero()).min(Self::one());
        NumCast::from(clamped * NumCast::from(255).unwrap()).unwrap()
    }
}

impl NormFloat for f32 {}
impl NormFloat for f64 {}

impl<T: NormFloat> ImageGA<T> {
    /// Creates a new ImageGA from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0 && data.dim().1 > 0);
        debug_assert!(data.dim().2 == 2);
        debug_assert!(data.iter().all(|&v| v >= T::zero() && v <= T::one()));
        Self { data }
    }

    /// Creates an empty image (all zeros) with alpha set to one.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0 && height > 0);
        let mut data = Array3::zeros((height, width, 2));
        data.slice_mut(s![.., .., 1]).fill(T::one());
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(width: usize, height: usize, value: [T; 2]) -> Self {
        debug_assert!(width > 0 && height > 0);
        debug_assert!(value.iter().all(|&v| v >= T::zero() && v <= T::one()));
        let mut data = Array3::zeros((height, width, 2));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        Self { data }
    }

    /// Creates an ImageGA from two grayscale layers.
    pub fn from_layers(layers: [Array2<T>; 2]) -> Self {
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

    /// Gets the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 2);
        self.data[[coords[1], coords[0], component]]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 2);
        debug_assert!(value >= T::zero() && value <= T::one());
        self.data[[coords[1], coords[0], component]] = value;
    }

    /// Gets the pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 2] {
        let pixel_slice = self.data.slice(s![coords[1], coords[0], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Sets the pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 2]) {
        debug_assert!(pixel.iter().all(|&v| v >= T::zero() && v <= T::one()));
        let mut view = self.data.slice_mut(s![coords[1], coords[0], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Gets a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
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

    /// Saves the normalized image as a PNG (converting [0,1] to u8).
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
        encoder.set_color(ColorType::GrayscaleAlpha);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        // Flip vertically and convert to u8.
        let flipped = self.data.slice(s![..;-1, .., ..]);
        let data: Vec<u8> = flipped.iter().map(|&v| v.to_u8()).collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads a PNG image and converts it to a normalized ImageGA.
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
        let _info = reader.next_frame(&mut buffer).map_err(|err| {
            ImageError::from_message(format!("Failed to decode PNG frame: {}", err))
        })?;

        // Check for GrayscaleAlpha with 8-bit depth.
        // Note: info isn't used further here; we rely on dimensions computed from buffer.
        let width = reader.info().width as usize;
        let height = reader.info().height as usize;
        let channels = 2;
        let total_bytes = width * height * channels;
        let data_vec = buffer[..total_bytes].to_vec();

        // Convert u8 data to normalized T.
        let image_array_u8 = Array3::from_shape_vec((height, width, channels), data_vec)
            .map_err(|err| ImageError::from_message(format!("Array creation error: {}", err)))?;
        let divisor = T::from(255).unwrap();
        let image_array = image_array_u8.map(|&v| T::from(v).unwrap() / divisor);

        // Flip vertically.
        let data = image_array.slice(s![..;-1, .., ..]).to_owned();
        Ok(Self { data })
    }
}

impl<T: NormFloat> std::fmt::Display for ImageGA<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
