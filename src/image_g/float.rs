use ndarray::{s, Array2, Axis};
use num_traits::{Float, NumCast};
use png::{ColorType, Decoder, Encoder};
use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageG};

/// Trait to convert normalized values ([0,1]) to u8.
pub trait NormFloat: Float + NumCast {
    fn to_u8(self) -> u8 {
        let clamped = self.max(Self::zero()).min(Self::one());
        NumCast::from(clamped * NumCast::from(255).unwrap()).unwrap()
    }
}

impl NormFloat for f32 {}
impl NormFloat for f64 {}

impl<T: NormFloat> ImageG<T> {
    /// Creates a new ImageG from the provided data.
    pub fn new(data: Array2<T>) -> Self {
        debug_assert!(data.ncols() > 0);
        debug_assert!(data.nrows() > 0);
        debug_assert!(data.iter().all(|&v| v >= T::zero() && v <= T::one()));
        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let data = Array2::zeros((height, width));
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(width: usize, height: usize, value: [T; 1]) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        debug_assert!(value[0] >= T::zero() && value[0] <= T::one());
        let data = Array2::from_elem((height, width), value[0]);
        Self { data }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    /// Gets the value of a component (the only one) at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 1);
        self.data[[coords[1], coords[0]]]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 1);
        debug_assert!(value >= T::zero() && value <= T::one());
        self.data[[coords[1], coords[0]]] = value;
    }

    /// Gets the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 1] {
        [self.data[[coords[1], coords[0]]]]
    }

    /// Sets the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 1]) {
        debug_assert!(pixel[0] >= T::zero() && pixel[0] <= T::one());
        self.data[[coords[1], coords[0]]] = pixel[0];
    }

    /// Transposes the image.
    pub fn transpose(&mut self) {
        self.data = self.data.t().to_owned();
    }

    /// Flips the image vertically.
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    /// Flips the image horizontally.
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Rotates the image 90° clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.data = self.data.t().slice(s![.., ..;-1]).to_owned();
    }

    /// Rotates the image 90° anticlockwise.
    pub fn rotate_anticlockwise(&mut self) {
        self.data = self.data.t().slice(s![..;-1, ..]).to_owned();
    }

    /// Rotates the image 180°.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
    }

    /// Saves the image as a PNG in grayscale format.
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
        encoder.set_color(ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        // Flip vertically and convert to u8.
        let flipped = self.data.slice(s![..;-1, ..]);
        let data: Vec<u8> = flipped.iter().map(|&v| v.to_u8()).collect();

        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Loads a PNG grayscale image and converts it to normalized values.
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

        if info.color_type != ColorType::Grayscale || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 1;
        let total_bytes = width * height * channels;
        let data: Vec<u8> = buffer[..total_bytes].to_vec();

        let image = Array2::from_shape_vec((height, width), data).map_err(|err| {
            ImageError::from_message(format!("Failed to create image array: {}", err))
        })?;
        let flipped = image.slice(s![..;-1, ..]).to_owned();
        let data_t = flipped.mapv(|v| T::from(v).unwrap() / T::from(255).unwrap());
        Ok(Self { data: data_t })
    }
}

impl<T: NormFloat> std::fmt::Display for ImageG<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter().rev() {
            for &value in row {
                let pixel = value.to_u8();
                write!(f, "\x1b[48;2;{0};{0};{0}m  \x1b[0m", pixel)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
