use image::{ImageBuffer, Rgba};
use ndarray::Array2;

use std::path::Path;

/// A simple image representation.
/// Colours are stored as RGBA components in the range [0.0, 1.0].
pub struct Image {
    width: usize,
    height: usize,
    colours: Array2<Rgba<f32>>,
}

impl Image {
    /// Create a new image with the given dimensions and fill colour.
    pub fn new(width: usize, height: usize, fill_colour: [f32; 4]) -> Self {
        Self {
            width,
            height,
            colours: Array2::from_elem((height, width), Rgba(fill_colour)),
        }
    }

    /// Get the number of pixels along the x-axis.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the number of pixels along the y-axis.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the image data as a 2D array of colour components.
    pub fn as_2d_u8(&self) -> Vec<[u8; 4]> {
        self.colours
            .iter()
            .map(|c| {
                [
                    (c[0] * 255.0) as u8,
                    (c[1] * 255.0) as u8,
                    (c[2] * 255.0) as u8,
                    (c[3] * 255.0) as u8,
                ]
            })
            .collect()
    }

    /// Get the image data as a flat array of u8 components.
    pub fn as_1d_u8(&self) -> Vec<u8> {
        self.as_2d_u8().into_iter().flatten().collect()
    }

    /// Get the image data as a flat array of f32 components.
    pub fn as_1d_f32(&self) -> Vec<f32> {
        self.colours
            .iter()
            .flat_map(|&Rgba(c)| c.into_iter())
            .collect()
    }

    /// Get the image data as a 2D array of RGBA components.
    pub fn as_2d_rgba(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = self.colours.ncols() as u32;
        let height = self.colours.nrows() as u32;

        ImageBuffer::from_raw(width, height, self.as_1d_u8()).unwrap()
    }

    /// Save the image as a PNG file at the given path.
    /// The directory will be created if it doesn't exist.
    pub fn save<P: AsRef<Path>>(&self, path: P) {
        // Create the directory if it doesn't exist.
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }

        self.as_2d_rgba().save(path).expect("Failed to save image");
    }

    /// Load a PNG image from the given path.
    pub fn load(path: &str) -> Self {
        let image = image::open(path).expect("Failed to load image");

        let image = image.to_rgba8();

        let width = image.width() as usize;
        let height = image.height() as usize;

        let colours = Array2::from_shape_vec(
            (height, width),
            image
                .pixels()
                .map(|p| {
                    let [r, g, b, a] = p.0;
                    Rgba([
                        r as f32 / 255.0,
                        g as f32 / 255.0,
                        b as f32 / 255.0,
                        a as f32 / 255.0,
                    ])
                })
                .collect(),
        )
        .unwrap();

        Self {
            width,
            height,
            colours,
        }
    }
}
