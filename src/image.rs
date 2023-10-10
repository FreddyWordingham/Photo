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

    pub fn from_1d_f32(&mut self, data: &[f32]) {
        for (i, &c) in data.iter().enumerate() {
            let channel = i % 4;
            let x = i / 4 % self.width;
            let y = i / 4 / self.width;
            self.colours[[y, x]][channel] = c;
        }
    }

    /// Get the image data as a 2D array of RGBA components.
    pub fn as_2d_rgba(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let width = self.colours.ncols() as u32;
        let height = self.colours.nrows() as u32;

        ImageBuffer::from_raw(width, height, self.as_1d_u8()).unwrap()
    }

    /// Save the image as a PNG file at the given file path.
    /// The directory will be created if it doesn't exist.
    pub fn save(&self, file_path: &str) {
        // Create the directory if it doesn't exist.
        if let Some(parent) = Path::new(file_path).parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }

        self.as_2d_rgba()
            .save(file_path)
            .expect("Failed to save image");
    }

    /// Load a PNG image from the given file path.
    pub fn load(file_path: &str) -> Self {
        let image = image::open(file_path).expect("Failed to load image");

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

    /// Select all pixels within the given radius of the given position.
    pub fn select_circle(&self, x: usize, y: usize, r: usize) -> Vec<[usize; 2]> {
        let mut points = Vec::new();
        let radius_squared = (r as isize).pow(2);

        for dx in -(r as isize) as isize..=r as isize {
            let x_pos = x as isize + dx;
            if x_pos < 0 || x_pos >= self.height as isize {
                continue;
            }

            for dy in -(r as isize) as isize..=r as isize {
                let y_pos = y as isize + dy;
                if y_pos < 0 || y_pos >= self.width as isize {
                    continue;
                }

                let distance_squared = dx.pow(2) + dy.pow(2);
                if distance_squared <= radius_squared {
                    points.push([y_pos as usize, x_pos as usize]);
                }
            }
        }

        points
    }

    /// Draw a pixel at the given position with the given radius and colour.
    pub fn draw_pixel(&mut self, x: usize, y: usize, col: [f32; 4]) {
        self.colours[[y, x]] = Rgba([col[0] as f32, col[1] as f32, col[2] as f32, col[3] as f32]);
    }

    /// Draw a circle at the given position with the given radius and colour.
    pub fn draw_circle(&mut self, x: usize, y: usize, r: usize, col: [f32; 4]) {
        let points = self.select_circle(x, y, r);
        for point in points {
            self.draw_pixel(point[0], point[1], col);
        }
    }
}
