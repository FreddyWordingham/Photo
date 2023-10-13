use image::{ImageBuffer, Rgba};
use ndarray::Array2;
use termion;

#[derive(Clone)]
pub struct Image {
    /// Stored in row-major order.
    data: Array2<Rgba<f32>>,
}

impl Image {
    /// Create a new image with the given dimensions and fill colour.
    pub fn new(nrows: usize, ncols: usize, fill_rgba: [f32; 4]) -> Self {
        debug_assert!(nrows > 0);
        debug_assert!(ncols > 0);
        debug_assert!(fill_rgba.iter().all(|&c| c >= 0.0 && c <= 1.0));

        let data = Array2::from_elem((nrows, ncols), Rgba(fill_rgba));

        Self { data }
    }

    /// Save the image as a PNG file at the given file path.
    /// The directory will be created if it doesn't exist.
    pub fn save(&self, file_path: &str) {
        if let Some(parent) = std::path::Path::new(file_path).parent() {
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }

        let image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(
            self.ncols() as u32,
            self.nrows() as u32,
            self.as_slice()
                .into_iter()
                .map(|c| (*c * 255.0) as u8)
                .collect::<Vec<_>>(),
        )
        .unwrap();
        image.save(file_path).expect("Failed to save image");
    }

    /// Load a PNG image from the given file path.
    pub fn load(file_path: &str) -> Self {
        let png = image::open(file_path)
            .expect("Failed to load image")
            .to_rgba8();
        let (ncols, nrows) = png.dimensions();

        let data = Array2::from_shape_vec(
            (nrows as usize, ncols as usize),
            png.pixels()
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

        Self { data }
    }

    /// Get the number of pixels along the height.
    pub fn nrows(&self) -> usize {
        self.data.nrows()
    }

    /// Get the number of pixels the image width.
    pub fn ncols(&self) -> usize {
        self.data.ncols()
    }

    /// Print the image to the console.
    pub fn print(&self, rgb: [f32; 3]) {
        for row in self.data.outer_iter() {
            for rgba in row {
                let r = (rgb[0] * rgba[0] * 255.0) as u8;
                let g = (rgb[1] * rgba[1] * 255.0) as u8;
                let b = (rgb[2] * rgba[2] * 255.0) as u8;
                print!(
                    "{}{}",
                    termion::color::Fg(termion::color::Rgb(r, g, b)),
                    "██",
                );
            }
            println!();
        }
        println!("{}", termion::color::Fg(termion::color::Reset));
    }

    /// Print a single channel of the image to the console.
    pub fn print_channel(&self, channel: usize) {
        debug_assert!(channel < 4);

        for row in self.data.outer_iter() {
            for rgba in row {
                let value = rgba[channel];
                let int = ((rgba[channel] * 200.0) + 55.0) as u8;
                let colour = match channel {
                    0 => termion::color::Rgb(int, 0, 0),
                    1 => termion::color::Rgb(0, int, 0),
                    2 => termion::color::Rgb(0, 0, int),
                    3 => termion::color::Rgb(int, int, int),
                    _ => panic!("Invalid channel index"),
                };
                print!(
                    "{}{} {}",
                    termion::color::Fg(colour),
                    format_float(value),
                    termion::color::Fg(termion::color::Reset)
                );
            }
            println!();
        }
        println!("{}", termion::color::Fg(termion::color::Reset));
    }

    /// Get the image data as a Vec<u8> after converting.
    pub fn as_u8(&self) -> Vec<u8> {
        self.data
            .iter()
            .flat_map(|&Rgba([r, g, b, a])| {
                vec![
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                    (a * 255.0) as u8,
                ]
            })
            .collect()
    }

    /// Get the image data as a slice.
    pub fn as_slice(&self) -> &[f32] {
        let raw_ptr = self.data.as_ptr() as *const f32;
        unsafe { std::slice::from_raw_parts(raw_ptr, self.data.len() * 4) }
    }

    pub fn from_slice(&mut self, slice: &[f32]) {
        let raw_ptr = self.data.as_mut_ptr() as *mut f32;
        unsafe {
            std::ptr::copy_nonoverlapping(slice.as_ptr(), raw_ptr, slice.len());
        }
    }

    /// Get the image data as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        let raw_ptr = self.data.as_mut_ptr() as *mut f32;
        unsafe { std::slice::from_raw_parts_mut(raw_ptr, self.data.len() * 4) }
    }

    // Set a pixel's colour.
    pub fn set_pixel(&mut self, row: usize, col: usize, rgba: [f32; 4]) {
        debug_assert!(row < self.nrows());
        debug_assert!(col < self.ncols());
        debug_assert!(rgba.iter().all(|&c| c >= 0.0 && c <= 1.0));

        self.data[[row, col]] = Rgba(rgba);
    }

    // Set pixel within a given radius of a point to a given colour.
    pub fn set_circle(&mut self, row: usize, col: usize, radius: usize, rgba: [f32; 4]) {
        let points = self.select_circle(row, col, radius);
        for point in points {
            self.set_pixel(point[0], point[1], rgba);
        }
    }

    // Set pixel within a given bounding box a given colour.
    pub fn set_rectangle(
        &mut self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
        rgba: [f32; 4],
    ) {
        debug_assert!(row < self.nrows());
        debug_assert!(col < self.ncols());
        debug_assert!(height > 0);
        debug_assert!(width > 0);
        debug_assert!(rgba.iter().all(|&c| c >= 0.0 && c <= 1.0));

        let points = self.select_rectangle(row, col, height, width);
        for point in points {
            self.set_pixel(point[0], point[1], rgba);
        }
    }

    /// Select all pixels within the given radius of the given position.
    pub fn select_circle(&self, row: usize, col: usize, radius: usize) -> Vec<[usize; 2]> {
        debug_assert!(row < self.nrows());
        debug_assert!(col < self.ncols());
        debug_assert!(radius > 0);

        let mut points = Vec::new();

        let radius_squared = (radius as isize).pow(2);

        for row_delta in -(radius as isize) as isize..=radius as isize {
            let row_pos = row as isize + row_delta;
            if row_pos < 0 || row_pos >= self.nrows() as isize {
                continue;
            }

            for col_delta in -(radius as isize) as isize..=radius as isize {
                let col_pos = col as isize + col_delta;
                if col_pos < 0 || col_pos >= self.ncols() as isize {
                    continue;
                }

                let distance_squared = row_delta.pow(2) + col_delta.pow(2);
                if distance_squared <= radius_squared {
                    points.push([row_pos as usize, col_pos as usize]);
                }
            }
        }

        points
    }

    /// Select all pixels in a bounding box.
    pub fn select_rectangle(
        &self,
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) -> Vec<[usize; 2]> {
        debug_assert!(row < self.nrows());
        debug_assert!(col < self.ncols());
        debug_assert!(height > 0);
        debug_assert!(width > 0);

        let mut points = Vec::new();

        for row_delta in 0..height {
            let row_pos = row + row_delta;
            if row_pos >= self.nrows() {
                continue;
            }

            for col_delta in 0..width {
                let col_pos = col + col_delta;
                if col_pos >= self.ncols() {
                    continue;
                }

                points.push([row_pos, col_pos]);
            }
        }

        points
    }
}

fn format_float(f: f32) -> String {
    debug_assert!(f >= 0.0);
    debug_assert!(f <= 1.0);

    if f == 1.0 {
        return "1.".to_string();
    }
    format!("{:.1}", f)[1..].to_string()
}
