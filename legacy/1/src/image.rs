use image::{ImageBuffer, Rgba};
use ndarray::Array2;

use std::path::Path;

/// A simple image representation.
/// Colours are stored as RGBA components in the range [0.0, 1.0].
pub struct Image {
    colours: Array2<Rgba<f32>>,
}

impl Image {
    /// Create a new image with the given dimensions and fill colour.
    pub fn new(nrows: usize, ncols: usize, fill_colour: [f32; 4]) -> Self {
        let arr = Array2::from_elem((nrows, ncols), Rgba(fill_colour));
        println!("{} {}", arr.nrows(), arr.ncols());
        Self { colours: arr }
    }

    /// Get the number of pixels along the height.
    pub fn nrows(&self) -> usize {
        self.colours.nrows()
    }

    /// Get the number of pixels the image width.
    pub fn ncols(&self) -> usize {
        self.colours.ncols()
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
        for (i, &col) in data.iter().enumerate() {
            let channel = i % 4;
            let idx_1d = i / 4;
            let r = idx_1d / self.ncols();
            let c = idx_1d % self.ncols();
            self.colours[[r, c]][channel] = col;
        }
    }

    /// Get the image data as a 2D array of RGBA components.
    pub fn as_2d_rgba(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        ImageBuffer::from_raw(self.ncols() as u32, self.nrows() as u32, self.as_1d_u8()).unwrap()
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
        let (ncols, nrows) = image.dimensions();

        println!("{} {}", nrows, ncols);
        println!("{} {}", nrows, ncols);
        println!("{} {}", nrows, ncols);

        let colours = Array2::from_shape_vec(
            (nrows as usize, ncols as usize),
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

        Self { colours }
    }

    /// Select all pixels within the given radius of the given position.
    pub fn select_circle(&self, r: usize, c: usize, rad: usize) -> Vec<[usize; 2]> {
        let mut points = Vec::new();

        let radius_squared = (rad as isize).pow(2);

        for dr in -(rad as isize) as isize..=rad as isize {
            let r_pos = r as isize + dr;
            if r_pos < 0 || r_pos >= self.nrows() as isize {
                continue;
            }

            for dc in -(rad as isize) as isize..=rad as isize {
                let c_pos = c as isize + dc;
                if c_pos < 0 || c_pos >= self.ncols() as isize {
                    continue;
                }

                let distance_squared = dr.pow(2) + dc.pow(2);
                if distance_squared <= radius_squared {
                    points.push([r_pos as usize, c_pos as usize]);
                }
            }
        }

        points
    }

    /// Draw a pixel at the given position with the given radius and colour.
    pub fn draw_pixel(&mut self, r: usize, c: usize, col: [f32; 4]) {
        self.colours[[r, c]] = Rgba([col[0] as f32, col[1] as f32, col[2] as f32, col[3] as f32]);
    }

    /// Draw a circle at the given position with the given radius and colour.
    pub fn draw_circle(&mut self, r: usize, c: usize, rad: usize, col: [f32; 4]) {
        let points = self.select_circle(r, c, rad);
        for point in points {
            self.draw_pixel(point[0], point[1], col);
        }
    }
}
