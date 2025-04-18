use ndarray::{
    Array2, Array3, ArrayBase, ArrayView3, ArrayViewMut3, Axis, Data, Ix2, arr1, s, stack,
};
use num_traits::{One, Zero};

use crate::{Direction, Transformation};

/// A grayscale image with transparency.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageGA<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

impl<T: Copy + PartialOrd + Zero + One> ImageGA<T> {
    /// Creates a new ImageGA from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 2);
        Self { data }
    }

    /// Creates an empty image (all zeros) with alpha set to one.
    pub fn empty(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let mut data = Array3::zeros((resolution[0], resolution[1], 2));
        data.slice_mut(s![.., .., 1]).fill(T::one());
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(resolution: [usize; 2], value: [T; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let mut data = Array3::zeros((resolution[0], resolution[1], 2));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        Self { data }
    }

    /// Creates an ImageGA from two grayscale layers.
    pub fn from_component_layers(layers: [Array2<T>; 2]) -> Self {
        debug_assert!(layers.iter().all(|layer| layer.ncols() > 0));
        debug_assert!(layers.iter().all(|layer| layer.nrows() > 0));
        debug_assert!(layers.iter().all(|layer| layer.dim() == layers[0].dim()));
        let data =
            stack(Axis(2), &[layers[0].view(), layers[1].view()]).expect("Failed to stack layers");
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

        let mut data = Array3::zeros((rows * tile_h, cols * tile_w, 2));
        for ((r, c), tile) in tiles.indexed_iter() {
            let sy = r * tile_h;
            let sx = c * tile_w;
            data.slice_mut(s![sy..sy + tile_h, sx..sx + tile_w, ..])
                .assign(&tile.data);
        }
        ImageGA::new(data)
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Gets the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 2);
        self.data[[coords[0], coords[1], component]]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 2);
        self.data[[coords[0], coords[1], component]] = value;
    }

    /// Gets the pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 2] {
        let pixel_slice = self.data.slice(s![coords[0], coords[1], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Sets the pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 2]) {
        let mut view = self.data.slice_mut(s![coords[0], coords[1], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Gets a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
        debug_assert!(component < 2);
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
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> ImageGA<T> {
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
    pub fn interior(&self, border_size: usize) -> ImageGA<T> {
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
}

mod float;
mod u8;
