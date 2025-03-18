use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis, s};
use num_traits::Zero;

use crate::{Direction, Transformation};

/// An image with a complete pixel in each element.
#[derive(Debug, Clone, PartialEq)]
pub struct Image<T> {
    /// Image data stored in row-major order.
    pub data: Array2<T>,
}

impl<T: Clone + Default + Zero> Image<T> {
    /// Creates a new Image from the provided data.
    pub fn new(data: Array2<T>) -> Self {
        debug_assert!(data.ncols() > 0);
        debug_assert!(data.nrows() > 0);
        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let data = Array2::default(resolution);
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(resolution: [usize; 2], value: T) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let data = Array2::from_elem(resolution, value);
        Self { data }
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> T {
        self.data[coords].clone()
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: T) {
        self.data[coords] = pixel;
    }

    /// Return a new image with the transformation applied.
    pub fn transform(&self, transform: Transformation) -> Self {
        let mut image = self.clone();
        image.transform_inplace(transform);
        image
    }

    /// Apply a transformation to the image.
    pub fn transform_inplace(&mut self, transform: Transformation) {
        // Determine if the image is square.
        let (rows, cols) = (self.data.nrows(), self.data.ncols());
        let is_square = rows == cols;

        match transform {
            Transformation::Identity => { /* do nothing */ }
            Transformation::Rotate90 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(1)); // horizontal flip
                } else {
                    self.data = self.data.t().to_owned().slice(s![.., ..;-1]).to_owned();
                }
            }
            Transformation::Rotate180 => {
                // These can be done in-place regardless of shape.
                self.data.invert_axis(Axis(0));
                self.data.invert_axis(Axis(1));
            }
            Transformation::Rotate270 => {
                if is_square {
                    self.data.swap_axes(0, 1);
                    self.data.invert_axis(Axis(0)); // vertical flip
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

    /// Extract a portion of the image.
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> Image<T> {
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

    /// Create a view of the images border.
    pub fn view_border(&self, direction: Direction, size: usize) -> ArrayView2<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice(s![0..size, ..]),
            Direction::East => self.data.slice(s![.., (self.width() - size)..]),
            Direction::South => self.data.slice(s![(self.height() - size).., ..]),
            Direction::West => self.data.slice(s![.., 0..size]),
        }
    }

    /// Create a mutable view of the images border.
    pub fn view_border_mut(&mut self, direction: Direction, size: usize) -> ArrayViewMut2<T> {
        debug_assert!(size > 0);
        match direction {
            Direction::North => self.data.slice_mut(s![0..size, ..]),
            Direction::East => self.data.slice_mut(s![.., (self.width() - size)..]),
            Direction::South => self.data.slice_mut(s![(self.height() - size).., ..]),
            Direction::West => self.data.slice_mut(s![.., 0..size]),
        }
    }
}

mod lin_srgb;
mod lin_srgba;
