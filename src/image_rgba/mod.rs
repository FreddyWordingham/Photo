use ndarray::{
    Array2, Array3, ArrayBase, ArrayView3, ArrayViewMut3, Axis, Data, Ix2, arr1, s, stack,
};
use num_traits::{One, Zero};
use std::fmt::Display;

use crate::{Direction, ImageError, Transformation};

/// A colour image with transparency.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageRGBA<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

impl<T: Copy + PartialOrd + Zero + One + Display> ImageRGBA<T> {
    /// Creates a new ImageRGBA from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 4);
        Self { data }
    }

    /// Creates an empty image (all zeros) with alpha set to one.
    pub fn empty(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let mut data = Array3::zeros((resolution[0], resolution[1], 4));
        data.slice_mut(s![.., .., 3]).fill(T::one());
        Self { data }
    }

    /// Creates an image filled with a constant RGBA value.
    pub fn filled(resolution: [usize; 2], value: [T; 4]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let mut data = Array3::zeros((resolution[0], resolution[1], 4));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        data.slice_mut(s![.., .., 2]).fill(value[2]);
        data.slice_mut(s![.., .., 3]).fill(value[3]);
        Self { data }
    }

    /// Creates an ImageRGBA from four layers (red, green, blue, alpha).
    pub fn from_component_layers(layers: [Array2<T>; 4]) -> Self {
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

    /// Creates an ImageRGBA from a 2D grid of tiles.
    pub fn from_tiles<D>(tiles: &ArrayBase<D, Ix2>) -> Self
    where
        D: Data<Elem = Self>,
    {
        assert!(!tiles.is_empty(), "tiles must not be empty");
        let (rows, cols) = tiles.dim();
        let tile_h = tiles[(0, 0)].height();
        let tile_w = tiles[(0, 0)].width();
        assert!(
            tile_h > 0 && tile_w > 0,
            "tiles must have positive dimensions"
        );

        let mut data = Array3::zeros((rows * tile_h, cols * tile_w, 4));
        for ((r, c), tile) in tiles.indexed_iter() {
            let sy = r * tile_h;
            let sx = c * tile_w;
            data.slice_mut(s![sy..sy + tile_h, sx..sx + tile_w, ..])
                .assign(&tile.data);
        }
        ImageRGBA::new(data)
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Get the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 4);
        self.data[[coords[0], coords[1], component]]
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 4);
        self.data[[coords[0], coords[1], component]] = value;
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 4] {
        let pixel_slice = self.data.slice(s![coords[0], coords[1], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice is not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 4]) {
        let mut view = self.data.slice_mut(s![coords[0], coords[1], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Get a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
        debug_assert!(component < 4);
        self.data.slice(s![.., .., component]).to_owned()
    }

    /// Return a new image with the transformation applied.
    pub fn transform(&self, transform: Transformation) -> Self {
        let mut image = self.clone();
        image.transform_inplace(transform);
        image
    }

    /// Apply a transformation to the image.
    pub fn transform_inplace(&mut self, transform: Transformation) {
        // Get spatial dimensions (assumes data shape is [rows, cols, channels])
        let shape = self.data.shape();
        let (rows, cols) = (shape[0], shape[1]);
        let is_square = rows == cols;

        match transform {
            Transformation::Identity => { /* do nothing */ }
            Transformation::Rotate90 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(1)); // horizontal flip
                } else {
                    self.data = self
                        .data
                        .view()
                        .permuted_axes([1, 0, 2])
                        .to_owned()
                        .slice(s![.., ..;-1, ..])
                        .to_owned();
                }
            }
            Transformation::Rotate180 => {
                self.data.invert_axis(Axis(0));
                self.data.invert_axis(Axis(1));
            }
            Transformation::Rotate270 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(0)); // vertical flip
                } else {
                    self.data = self
                        .data
                        .view()
                        .permuted_axes([1, 0, 2])
                        .to_owned()
                        .slice(s![..;-1, .., ..])
                        .to_owned();
                }
            }
            Transformation::FlipHorizontal => {
                self.data.invert_axis(Axis(1));
            }
            Transformation::FlipVertical => {
                self.data.invert_axis(Axis(0));
            }
            Transformation::FlipDiagonal => {
                if is_square {
                    self.data.swap_axes(0, 1);
                } else {
                    self.data = self.data.view().permuted_axes([1, 0, 2]).to_owned();
                }
            }
            Transformation::FlipAntiDiagonal => {
                if is_square {
                    self.data.invert_axis(Axis(0));
                    self.data.invert_axis(Axis(1));
                    self.data.swap_axes(0, 1);
                } else {
                    self.data = self
                        .data
                        .slice(s![..;-1, ..;-1, ..])
                        .to_owned()
                        .view()
                        .permuted_axes([1, 0, 2])
                        .to_owned();
                }
            }
        }
    }

    /// Extract a portion of the image.
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> ImageRGBA<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        Self::new(
            self.data
                .slice(s![
                    start[0]..start[0] + size[0],
                    start[1]..start[1] + size[1],
                    ..
                ])
                .to_owned(),
        )
    }

    /// Create a view to a portion of the image.
    pub fn view(&self, start: [usize; 2], size: [usize; 2]) -> ArrayView3<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        self.data.slice(s![
            start[0]..start[0] + size[0],
            start[1]..start[1] + size[1],
            ..
        ])
    }

    /// Create a mutable view to a portion of the image.
    pub fn view_mut(&mut self, start: [usize; 2], size: [usize; 2]) -> ArrayViewMut3<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        self.data.slice_mut(s![
            start[0]..start[0] + size[0],
            start[1]..start[1] + size[1],
            ..
        ])
    }

    /// Create a view of the image's border.
    pub fn view_border(&self, direction: Direction, size: usize) -> ArrayView3<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice(s![0..size, .., ..]),
            Direction::East => self.data.slice(s![.., (self.width() - size).., ..]),
            Direction::South => self.data.slice(s![(self.height() - size).., .., ..]),
            Direction::West => self.data.slice(s![.., 0..size, ..]),
        }
    }

    /// Create a mutable view of the image's border.
    pub fn view_border_mut(&mut self, direction: Direction, size: usize) -> ArrayViewMut3<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice_mut(s![0..size, .., ..]),
            Direction::East => self.data.slice_mut(s![.., (self.width() - size).., ..]),
            Direction::South => self.data.slice_mut(s![(self.height() - size).., .., ..]),
            Direction::West => self.data.slice_mut(s![.., 0..size, ..]),
        }
    }

    /// Get a copy of the interior of the image.
    pub fn interior(&self, border_size: usize) -> ImageRGBA<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        Self::new(
            self.data
                .slice(s![
                    border_size..(height - border_size),
                    border_size..(width - border_size),
                    ..
                ])
                .to_owned(),
        )
    }

    /// Create a view of the image's interior.
    pub fn view_interior(&self, border_size: usize) -> ArrayView3<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        self.data.slice(s![
            border_size..(height - border_size),
            border_size..(width - border_size),
            ..
        ])
    }

    /// Create a mutable view of the image's interior.
    pub fn view_interior_mut(&mut self, border_size: usize) -> ArrayViewMut3<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        self.data.slice_mut(s![
            border_size..(height - border_size),
            border_size..(width - border_size),
            ..
        ])
    }

    /// Create an array of sub-tiles in the image.
    pub fn extract_tiles(&self, tile_size: usize, overlap: usize) -> Array2<Self> {
        let (height, width) = (self.height(), self.width());
        debug_assert!(overlap < tile_size);
        debug_assert!(height >= tile_size);
        debug_assert!(width >= tile_size);
        debug_assert_eq!(
            (width - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );
        debug_assert_eq!(
            (height - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );

        let num_horizontal_tiles = (width - overlap) / (tile_size - overlap);
        let num_vertical_tiles = (height - overlap) / (tile_size - overlap);

        let step_size = tile_size - overlap;
        Array2::from_shape_fn((num_vertical_tiles, num_horizontal_tiles), |(y, x)| {
            let start_y = y * step_size;
            let start_x = x * step_size;
            self.extract([start_y, start_x], [tile_size, tile_size])
        })
    }

    /// Converts the image into a Vec of display lines.
    fn to_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.height());
        for row in self.data.outer_iter() {
            let mut line = String::new();
            for pixel in row.outer_iter() {
                let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                // Build a cell with background color based on pixel RGBA.
                use std::fmt::Write;
                write!(line, "\x1b[48;2;{r};{g};{b};{a}m  \x1b[0m").unwrap();
            }
            lines.push(line);
        }
        lines
    }

    /// Create a view of tiles in the image.
    pub fn view_tiles(&self, tile_size: usize, overlap: usize) -> Array2<ArrayView3<T>> {
        let (height, width) = (self.height(), self.width());
        debug_assert!(overlap < tile_size);
        debug_assert!(height >= tile_size);
        debug_assert!(width >= tile_size);
        debug_assert_eq!(
            (width - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );
        debug_assert_eq!(
            (height - overlap) % (tile_size - overlap),
            0,
            "Image must contain an integer number of tiles"
        );

        let num_horizontal_tiles = (width - overlap) / (tile_size - overlap);
        let num_vertical_tiles = (height - overlap) / (tile_size - overlap);

        let step_size = tile_size - overlap;
        Array2::from_shape_fn((num_vertical_tiles, num_horizontal_tiles), |(y, x)| {
            let start_y = y * step_size;
            let start_x = x * step_size;
            self.view([start_y, start_x], [tile_size, tile_size])
        })
    }

    /// Print a grid of ImageRGBA references in a 2D array.
    /// The grid width is determined by the terminal width divided by the image's printed width plus the gap.
    pub fn print_image_grid(images: &[&Self], gap: usize) -> Result<(), ImageError> {
        // Get terminal width (fallback to 80 columns if needed; using 60 as a placeholder here)
        let term_width = if let Some((w, _h)) = term_size::dimensions() {
            w
        } else {
            80 // Fallback width if terminal size cannot be determined
        };

        // Ensure there's at least one image.
        let first = images.first().ok_or_else(|| {
            ImageError::from_message("No images provided for grid display.".to_string())
        })?;
        let img_width = first.width(); // image width in pixels
        // Each pixel is printed using two characters ("  ")
        let cell_width = img_width * 2;

        // Calculate images per row considering the gap between each image.
        let images_per_row = if term_width < cell_width {
            1
        } else {
            (term_width + (2 * gap)) / (cell_width + (2 * gap))
        };

        // Process images by chunks (each chunk forms a row)
        for row in images.chunks(images_per_row) {
            // Convert each image in the current row to its lines.
            let lines_vec: Vec<Vec<String>> = row.iter().map(|img| img.to_lines()).collect();
            // Determine the row height (assumes all images have equal height, otherwise use the maximum).
            let row_height = lines_vec.iter().map(|lines| lines.len()).max().unwrap_or(0);
            // Print the row line by line.
            for line_idx in 0..row_height {
                for (i, lines) in lines_vec.iter().enumerate() {
                    // Print the gap before each tile except the first.
                    if i > 0 {
                        print!("{:width$}", "", width = gap * 2);
                    }
                    if line_idx < lines.len() {
                        print!("{}", lines[line_idx]);
                    } else {
                        // Fill in with spaces if an image has fewer lines.
                        print!("{:width$}", "", width = cell_width);
                    }
                }
                println!();
            }
            for _ in 0..gap {
                println!(); // Add a gap between rows.
            }
        }
        Ok(())
    }

    /// Print a grid of ImageRGBA references with captions in a 2D array.
    /// Each tuple contains a caption and an image reference.
    /// The grid width is determined by the terminal width divided by the image's printed width plus the gap.
    pub fn print_image_grid_with_caption(
        images_with_caption: &[(&Self, String)],
        gap: usize,
    ) -> Result<(), ImageError> {
        // Get terminal width (fallback to 80 columns if needed; using 60 as a placeholder here)
        let term_width = if let Some((w, _h)) = term_size::dimensions() {
            w
        } else {
            60 // Fallback width if terminal size cannot be determined
        };

        // Ensure there's at least one image.
        let first = images_with_caption.first().ok_or_else(|| {
            ImageError::from_message("No images provided for grid display.".to_string())
        })?;
        let img_width = first.0.width(); // image width in pixels
        // Each pixel prints as two characters ("  ")
        let cell_width = img_width * 2;

        // Calculate images per row considering the gap between each image.
        let images_per_row = if term_width < cell_width {
            1
        } else {
            (term_width + (2 * gap)) / (cell_width + (2 * gap))
        };

        // Process each chunk (row) of images with captions.
        for row in images_with_caption.chunks(images_per_row) {
            // Convert each image in the current row into its display lines.
            let lines_vec: Vec<Vec<String>> = row.iter().map(|(img, _)| img.to_lines()).collect();
            // Find the maximum height in this row (assumes equal heights, else use the max).
            let row_height = lines_vec.iter().map(|lines| lines.len()).max().unwrap_or(0);

            // Print each line of the image grid.
            for line_idx in 0..row_height {
                for (i, lines) in lines_vec.iter().enumerate() {
                    if i > 0 {
                        print!("{:width$}", "", width = gap * 2);
                    }
                    if line_idx < lines.len() {
                        print!("{}", lines[line_idx]);
                    } else {
                        // Fill in with spaces if an image has fewer lines.
                        print!("{:width$}", "", width = cell_width);
                    }
                }
                println!();
            }

            // Print a single line with captions centered below each image.
            for (i, (_, caption)) in row.iter().enumerate() {
                if i > 0 {
                    print!("{:width$}", "", width = gap * 2);
                }
                // If the caption is longer than cell_width, trim it; otherwise, center align.
                let caption_print = if caption.len() > cell_width {
                    caption[..cell_width].to_string()
                } else {
                    format!("{:^width$}", caption, width = cell_width)
                };
                print!("{}", caption_print);
            }
            println!();

            // Add a vertical gap between rows.
            for _ in 0..gap {
                println!();
            }
        }
        Ok(())
    }
}

mod float;
mod u8;
