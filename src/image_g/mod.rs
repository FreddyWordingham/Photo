use enterpolation::Merge;
use ndarray::{Array2, ArrayBase, ArrayView2, ArrayViewMut2, Axis, Data, Ix2, s};
use num_traits::{Float, FromPrimitive, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::{ColourMap, Direction, Image, Transformation, colour_map::ColorFromHex};

/// An opaque grayscale image.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageG<T> {
    /// Image data stored in row-major order.
    pub data: Array2<T>,
}

impl<T: Copy + PartialOrd + Zero> ImageG<T> {
    /// Creates a new ImageG from the provided data.
    pub fn new(data: Array2<T>) -> Self {
        debug_assert!(data.ncols() > 0);
        debug_assert!(data.nrows() > 0);
        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let data = Array2::zeros(resolution);
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(resolution: [usize; 2], value: [T; 1]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let data = Array2::from_elem(resolution, value[0]);
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

        let mut data = Array2::zeros((rows * tile_h, cols * tile_w));
        for ((r, c), tile) in tiles.indexed_iter() {
            let sy = r * tile_h;
            let sx = c * tile_w;
            data.slice_mut(s![sy..sy + tile_h, sx..sx + tile_w])
                .assign(&tile.data);
        }
        ImageG::new(data)
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    /// Gets the value of a component (the only one) at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 1);
        self.data[coords]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 1);
        self.data[coords] = value;
    }

    /// Gets the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 1] {
        [self.data[coords]]
    }

    /// Sets the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 1]) {
        self.data[coords] = pixel[0];
    }

    /// Return a new image with the transformation applied.
    pub fn transform(&self, transform: Transformation) -> Self {
        let mut image = self.clone();
        image.transform_inplace(transform);
        image
    }

    /// Apply a transformation to the image.
    pub fn transform_inplace(&mut self, transform: Transformation) {
        let (rows, cols) = (self.data.nrows(), self.data.ncols());
        let is_square = rows == cols;

        match transform {
            Transformation::Identity => { /* do nothing */ }
            Transformation::Rotate90 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(1));
                } else {
                    self.data = self.data.t().to_owned().slice(s![.., ..;-1]).to_owned();
                }
            }
            Transformation::Rotate180 => {
                self.data.invert_axis(Axis(0));
                self.data.invert_axis(Axis(1));
            }
            Transformation::Rotate270 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(0));
                } else {
                    self.data = self.data.t().to_owned().slice(s![..;-1, ..]).to_owned();
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
                    self.data = self.data.t().to_owned();
                }
            }
            Transformation::FlipAntiDiagonal => {
                if is_square {
                    self.data.invert_axis(Axis(0));
                    self.data.invert_axis(Axis(1));
                    self.data.swap_axes(0, 1);
                } else {
                    self.data = self.data.slice(s![..;-1, ..;-1]).to_owned().t().to_owned();
                }
            }
        }
    }

    /// Colourize a grayscale image using a colour map.
    pub fn colourize<C>(self, cmap: &ColourMap<T, C>) -> Image<C>
    where
        T: Float + FromPrimitive + Debug,
        C: ColorFromHex<T>
            + Default
            + Debug
            + Copy
            + Add<Output = C>
            + Sub<Output = C>
            + Mul<T, Output = C>
            + Div<T, Output = C>
            + Merge<T>,
    {
        let mut data = Array2::default(self.data.dim());
        for (out, &value) in data.iter_mut().zip(self.data.iter()) {
            *out = cmap.sample(value);
        }
        Image { data }
    }

    /// Extract a portion of the image.
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> ImageG<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        Self::new(
            self.data
                .slice(s![
                    start[0]..start[0] + size[0],
                    start[1]..start[1] + size[1]
                ])
                .to_owned(),
        )
    }

    /// Create a view to a portion of the image.
    pub fn view(&self, start: [usize; 2], size: [usize; 2]) -> ArrayView2<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        self.data.slice(s![
            start[0]..start[0] + size[0],
            start[1]..start[1] + size[1]
        ])
    }

    /// Create a mutable view to a portion of the image.
    pub fn view_mut(&mut self, start: [usize; 2], size: [usize; 2]) -> ArrayViewMut2<T> {
        debug_assert!(start[0] + size[0] <= self.height());
        debug_assert!(start[1] + size[1] <= self.width());
        debug_assert!(size.iter().all(|&s| s > 0));
        self.data.slice_mut(s![
            start[0]..start[0] + size[0],
            start[1]..start[1] + size[1]
        ])
    }

    /// Create a view of the image's border.
    pub fn view_border(&self, direction: Direction, size: usize) -> ArrayView2<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice(s![0..size, ..]),
            Direction::East => self.data.slice(s![.., (self.width() - size)..]),
            Direction::South => self.data.slice(s![(self.height() - size).., ..]),
            Direction::West => self.data.slice(s![.., 0..size]),
        }
    }

    /// Create a mutable view of the image's border.
    pub fn view_border_mut(&mut self, direction: Direction, size: usize) -> ArrayViewMut2<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice_mut(s![0..size, ..]),
            Direction::East => self.data.slice_mut(s![.., (self.width() - size)..]),
            Direction::South => self.data.slice_mut(s![(self.height() - size).., ..]),
            Direction::West => self.data.slice_mut(s![.., 0..size]),
        }
    }

    /// Get a copy of the interior of the image.
    pub fn interior(&self, border_size: usize) -> ImageG<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        Self::new(
            self.data
                .slice(s![
                    border_size..(height - border_size),
                    border_size..(width - border_size)
                ])
                .to_owned(),
        )
    }

    /// Create a view of the image's interior.
    pub fn view_interior(&self, border_size: usize) -> ArrayView2<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        self.data.slice(s![
            border_size..(height - border_size),
            border_size..(width - border_size)
        ])
    }

    /// Create a mutable view of the image's interior.
    pub fn view_interior_mut(&mut self, border_size: usize) -> ArrayViewMut2<T> {
        let height = self.height();
        let width = self.width();
        debug_assert!(height > 2 * border_size);
        debug_assert!(width > 2 * border_size);
        self.data.slice_mut(s![
            border_size..(height - border_size),
            border_size..(width - border_size)
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
    pub fn view_tiles(&self, tile_size: usize, overlap: usize) -> Array2<ArrayView2<T>> {
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
