//! ## `image` representation and manipulation.
//!
//! The `image` module `Image` struct and related functionality
//! for working with multi-channel images, including operations like
//! region copying, tiling, stacking, and border handling.

use core::{
    mem::replace,
    ops::{Index, IndexMut},
};
use nav::Direction;
use ndarray::{
    Array2, Array3, ArrayBase, ArrayView1, ArrayView2, ArrayView3, ArrayViewMut3, Axis, Data, Ix1, Ix2, Ix3, concatenate, s,
    stack,
};
use num_traits::Zero;

use crate::Channels;

/// Representation of an image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image<T> {
    /// The format of this image.
    pub format: Channels,
    /// Image data stored in row-major order with channels as the third dimension.
    pub data: Array3<T>,
}

impl<T> Image<T> {
    /// Creates a new `Image` from the provided data.
    ///
    /// # Panics
    ///
    /// Panics if the height or width of the data is not positive.
    /// Panics if the number of channels is not between 1 and 4.
    #[expect(clippy::expect_used, reason = "Number of channels is checked in the function.")]
    #[inline]
    #[must_use]
    pub fn new<S>(data: &ArrayBase<S, Ix3>) -> Self
    where
        S: Data<Elem = T>,
        T: Clone,
    {
        let dim = data.dim();
        assert!(dim.0 > 0, "Image height must be positive");
        assert!(dim.1 > 0, "Image width must be positive");
        let format = Channels::from_num_channels(dim.2).expect("Number of channels must be between 1 and 4");

        // Convert to owned Array3 regardless of input type
        let owned_data = data.to_owned();

        Self {
            format,
            data: owned_data,
        }
    }

    /// Creates an empty (all zeros) `Image` with the given dimensions and format.
    ///
    /// # Panics
    ///
    /// Panics if the height or width is not positive.
    #[inline]
    #[must_use]
    pub fn empty((height, width): (usize, usize), format: Channels) -> Self
    where
        T: Zero + Clone,
    {
        assert!(height > 0, "Height must be positive");
        assert!(width > 0, "Width must be positive");
        let data = Array3::zeros((height, width, format.num_channels()));
        Self { format, data }
    }

    /// Creates an `Image` filled with a constant value per channel.
    ///
    /// # Panics
    ///
    /// Panics if the number of channels in `values` is not between 1 and 4.
    #[expect(clippy::expect_used, reason = "Number of channels is checked in the function.")]
    #[inline]
    #[must_use]
    pub fn filled((height, width): (usize, usize), values: &[T]) -> Self
    where
        T: Zero + Clone,
    {
        assert!(height > 0, "Height must be positive");
        assert!(width > 0, "Width must be positive");
        let format = Channels::from_num_channels(values.len()).expect("Number of channels must be between 1 and 4");

        let mut data = Array3::zeros((height, width, format.num_channels()));
        for (channel, value) in values.iter().enumerate().take(format.num_channels()) {
            data.slice_mut(s![.., .., channel]).fill(value.clone());
        }

        Self { format, data }
    }

    /// Creates an `Image` from component layers.
    ///
    /// # Panics
    ///
    /// Panics if the number of layers is not between 1 and 4.
    /// Panics if the height or width of the layers is not positive.
    /// Panics if the layers do not have the same dimensions.
    #[expect(clippy::expect_used, reason = "Stacking error would arise from within third party library.")]
    #[inline]
    #[must_use]
    pub fn from_layers<S>(layers: &[ArrayBase<S, Ix2>]) -> Self
    where
        S: Data<Elem = T>,
        T: Clone,
    {
        let format = Channels::from_num_channels(layers.len()).expect("Number of layers must be between 1 and 4");

        // Ensure we have at least one layer
        assert!(!layers.is_empty(), "At least one layer is required");

        let (height, width) = layers[0].dim();
        assert!(height > 0, "Image height must be positive");
        assert!(width > 0, "Image width must be positive");
        assert!(
            layers.iter().all(|layer| layer.dim() == (height, width)),
            "All layers must have the same dimensions"
        );

        // Create views of each layer
        let views: Vec<_> = layers.iter().map(ArrayBase::view).collect();

        // Stack the views along the channel axis
        let data = stack(Axis(2), &views).expect("Failed to stack layers");

        Self { format, data }
    }

    /// Combine images by stacking them vertically.
    ///
    /// # Panics
    ///
    /// Panics if the there are no images.
    /// Panics if the images do not all have the same width.
    /// Panics if the images do not all have the same format.
    /// Panics if the images do not all have positive height.
    #[expect(
        clippy::expect_used,
        reason = "Concatenate error would arise from within third party library."
    )]
    #[inline]
    #[must_use]
    pub fn vstack(images: &[Self]) -> Self
    where
        T: Clone + Zero,
    {
        assert!(!images.is_empty(), "At least one image is required");
        let width = images[0].width();
        assert!(width > 0, "Image widths must be positive");
        assert!(
            images.iter().all(|image| image.width() == width),
            "All images must have the same width"
        );
        assert!(
            images.iter().all(|image| image.height() > 0),
            "All images must have positive height"
        );
        let format = images[0].format;
        assert!(
            images.iter().all(|image| image.format == format),
            "All images must have the same format"
        );

        let views: Vec<_> = images.iter().map(|img| img.data.view()).collect();
        let data = concatenate(Axis(0), &views).expect("concatenate failed");
        Self { format, data }
    }

    /// Combine images by stacking them horizontally.
    ///
    /// # Panics
    ///
    /// Panics if there are no images.
    /// Panics if the images do not all have the same height.
    /// Panics if the images do not all have the same format.
    /// Panics if the images do not all have positive width.
    #[expect(
        clippy::expect_used,
        reason = "Concatenate error would arise from within third party library."
    )]
    #[inline]
    #[must_use]
    pub fn hstack(images: &[Self]) -> Self
    where
        T: Clone + Zero,
    {
        assert!(!images.is_empty(), "At least one image is required");
        let height = images[0].height();
        assert!(height > 0, "Image heights must be positive");
        assert!(
            images.iter().all(|img| img.height() == height),
            "All images must have the same height"
        );
        let format = images[0].format;
        assert!(
            images.iter().all(|img| img.format == format),
            "All images must have the same format"
        );

        let views: Vec<_> = images.iter().map(|img| img.data.view()).collect();
        let data = concatenate(Axis(1), &views).expect("concatenate failed");
        Self { format, data }
    }

    /// Creates an `Image` from a 2D grid of `Image` tiles.
    ///
    /// # Panics
    ///
    /// Panics if the grid dimensions are not positive.
    /// Panics if the tiles do not all have the same dimensions.
    /// Panics if the tile width or height is not positive.
    /// Panics if the tiles do not all have the same format.
    #[expect(clippy::min_ident_chars, reason = "Variables `r` and `c` are very limited in scope.")]
    #[inline]
    pub fn stack<D>(tiles: &ArrayBase<D, Ix2>) -> Self
    where
        D: Data<Elem = Self>,
        T: Clone + Zero,
    {
        let (grid_height, grid_width) = tiles.dim();
        assert!(grid_height > 0, "Tile grid height must be positive");
        assert!(grid_width > 0, "Tile grid width must be positive");

        let first_tile = &tiles[(0, 0)];
        let (tile_height, tile_width) = first_tile.resolution();
        assert!(tile_height > 0, "Tile height must be positive");
        assert!(tile_width > 0, "Tile width must be positive");
        assert!(
            tiles.iter().all(|tile| tile.resolution() == (tile_height, tile_width)),
            "All tiles must have the same resolution",
        );

        let format = first_tile.format;
        assert!(
            tiles.iter().all(|tile| tile.format == format),
            "All tiles must have the same format"
        );

        let channels = format.num_channels();
        let mut data = Array3::zeros((grid_height * tile_height, grid_width * tile_width, channels));
        for ((row, col), tile) in tiles.indexed_iter() {
            let r = row * tile_height;
            let c = col * tile_width;
            data.slice_mut(s![r..(r + tile_height), c..(c + tile_width), ..])
                .assign(&tile.data);
        }

        Self { format, data }
    }

    /// Helper method to create a default-initialised image with the same format.
    #[inline]
    #[must_use]
    fn default_like(&self) -> Self
    where
        T: Clone + Zero,
    {
        let (height, width) = self.resolution();
        let num_channels = self.format.num_channels();

        Self {
            format: self.format,
            data: Array3::zeros((height, width, num_channels)),
        }
    }

    /// Returns the height of the image.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Returns the width of the image.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Returns the size of the image as (height, width).
    #[inline]
    #[must_use]
    pub fn resolution(&self) -> (usize, usize) {
        (self.data.dim().0, self.data.dim().1)
    }

    /// Returns the format of the image.
    #[inline]
    #[must_use]
    pub const fn format(&self) -> Channels {
        self.format
    }

    /// Get a layer of the image as a 2D view.
    ///
    /// # Panics
    ///
    /// Panics if the channel index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_channel(&self, channel: usize) -> ArrayView2<T> {
        assert!(channel < self.data.dim().2, "Channel index out of bounds");
        self.data.slice(s![.., .., channel])
    }

    /// Set a layer of the image.
    ///
    /// # Panics
    ///
    /// Panics if the channel index is out of bounds.
    /// Panics if the dimensions of the layer do not match the image dimensions.
    #[inline]
    pub fn set_channel<S>(&mut self, channel: usize, layer: &ArrayBase<S, Ix2>)
    where
        S: Data<Elem = T>,
        T: Clone,
    {
        assert!(channel < self.data.dim().2, "Channel index out of bounds");
        assert_eq!(
            layer.dim(),
            (self.data.dim().0, self.data.dim().1),
            "Layer dimensions do not match"
        );
        self.data.slice_mut(s![.., .., channel]).assign(layer);
    }
    /// Get a view of a pixel at the given coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the pixel index is out of bounds.
    #[inline]
    #[must_use]
    pub fn get_pixel(&self, (row, col): (usize, usize)) -> ArrayView1<T> {
        let (height, width, _) = self.data.dim();
        assert!(row < height && col < width, "Pixel index out of bounds");
        self.data.slice(s![row, col, ..])
    }

    /// Set a pixel at the given coordinates.
    ///
    /// # Panics
    ///
    /// Panics if the pixel index is out of bounds.
    /// Panics if the pixel length does not match the number of channels.
    #[inline]
    pub fn set_pixel<S>(&mut self, (row, col): (usize, usize), pixel: &ArrayBase<S, Ix1>)
    where
        S: Data<Elem = T>,
        T: Clone,
    {
        let (height, width, _) = self.data.dim();
        assert!(row < height && col < width, "Pixel index out of bounds");
        assert_eq!(
            pixel.len(),
            self.data.dim().2,
            "Pixel length does not match number of channels"
        );
        self.data.slice_mut(s![row, col, ..]).assign(pixel);
    }

    /// Get a view of a region of the `Image`.
    ///
    /// This returns a view rather than creating a new `Image`, which is more efficient
    /// when you only need to read from the region without modifying it.
    ///
    /// # Panics
    ///
    /// Panics if the region exceeds the `Image` dimensions.
    /// Panics if the region dimensions are not positive.
    #[inline]
    #[must_use]
    pub fn view_region(&self, (start_row, start_col): (usize, usize), (height, width): (usize, usize)) -> ArrayView3<T> {
        assert!(height > 0, "Region height must be positive");
        assert!(width > 0, "Region width must be positive");
        assert!(start_row + height <= self.height(), "Region exceeds image height");
        assert!(start_col + width <= self.width(), "Region exceeds image width");

        self.data
            .slice(s![start_row..(start_row + height), start_col..(start_col + width), ..])
    }

    /// Get a mutable view of a region of the `Image`.
    ///
    /// # Panics
    ///
    /// Panics if the region exceeds the `Image` dimensions.
    /// Panics if the region dimensions are not positive.
    #[inline]
    #[must_use]
    pub fn view_region_mut(
        &mut self,
        (start_row, start_col): (usize, usize),
        (height, width): (usize, usize),
    ) -> ArrayViewMut3<T> {
        assert!(height > 0, "Region height must be positive");
        assert!(width > 0, "Region width must be positive");
        assert!(start_row + height <= self.height(), "Region exceeds image height");
        assert!(start_col + width <= self.width(), "Region exceeds image width");

        self.data
            .slice_mut(s![start_row..(start_row + height), start_col..(start_col + width), ..])
    }

    /// Copy a region of the `Image` into a new `Image`.
    ///
    /// # Panics
    ///
    /// Panics if the region exceeds the `Image` dimensions.
    /// Panics if the region dimensions are not positive.
    #[inline]
    #[must_use]
    pub fn copy_region(&self, (start_row, start_col): (usize, usize), (height, width): (usize, usize)) -> Self
    where
        T: Clone,
    {
        assert!(height > 0, "Region height must be positive");
        assert!(width > 0, "Region width must be positive");
        assert!(start_row + height <= self.height(), "Region exceeds image height");
        assert!(start_col + width <= self.width(), "Region exceeds image width");

        let region = self
            .data
            .slice(s![start_row..(start_row + height), start_col..(start_col + width), ..]);
        Self::new(&region)
    }

    /// Copy a region of the `Image` into a new `Image`, with wrapping (toroidal) boundary conditions.
    ///
    /// # Panics
    ///
    /// Panics if the region dimensions are not positive.
    #[expect(clippy::expect_used, reason = "Wrapping distance is not expected to exceed isize::MAX.")]
    #[inline]
    #[must_use]
    pub fn copy_region_wrapped(&self, (start_row, start_col): (isize, isize), (height, width): (usize, usize)) -> Self
    where
        T: Clone,
    {
        /// Helper function to perform proper modulo for negative numbers
        #[expect(
            clippy::min_ident_chars,
            reason = "Variables `a` and `b` are used as mathematical variables."
        )]
        #[inline]
        const fn wrap_index(a: isize, b: isize) -> isize {
            a.rem_euclid(b)
        }

        assert!(height > 0, "Region height must be positive");
        assert!(width > 0, "Region width must be positive");

        let (self_height, self_width, channels) = self.data.dim();

        // Pre-calculate wrapped row and column indices for the region
        let wrapped_rows: Vec<usize> = (0..height)
            .map(|row| {
                usize::try_from(wrap_index(
                    start_row + isize::try_from(row).expect("height exceeds isize::MAX"),
                    isize::try_from(self_height).expect("self_height exceeds isize::MAX"),
                ))
                .expect("negative index after wrapping")
            })
            .collect();
        let wrapped_cols: Vec<usize> = (0..width)
            .map(|col| {
                usize::try_from(wrap_index(
                    start_col + isize::try_from(col).expect("width exceeds isize::MAX"),
                    isize::try_from(self_width).expect("self_width exceeds isize::MAX"),
                ))
                .expect("negative index after wrapping")
            })
            .collect();

        Self::new(&Array3::<T>::from_shape_fn(
            (height, width, channels),
            |(out_row, out_col, ch)| {
                let src_row = wrapped_rows[out_row];
                let src_col = wrapped_cols[out_col];
                self.data[(src_row, src_col, ch)].clone()
            },
        ))
    }

    /// View the interior region of the image, excluding a border of a given size.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than half the image dimensions.
    #[inline]
    #[must_use]
    pub fn view_interior(&self, border: usize) -> ArrayView3<T> {
        assert!(border > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        assert!((2 * border) < height, "Border size must be less than half the height");
        assert!((2 * border) < width, "Border size must be less than half the width");

        self.data.slice(s![border..(height - border), border..(width - border), ..])
    }

    /// Get a mutable view of the interior region of the image, excluding a border of a given size.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than half the image dimensions.
    #[inline]
    #[must_use]
    pub fn view_interior_mut(&mut self, border: usize) -> ArrayViewMut3<T> {
        assert!(border > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        assert!((2 * border) < height, "Border size must be less than half the height");
        assert!((2 * border) < width, "Border size must be less than half the width");

        self.data
            .slice_mut(s![border..(height - border), border..(width - border), ..])
    }

    /// Copy the interior region of the image, excluding a border of a given size.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than half the image dimensions.
    #[inline]
    #[must_use]
    pub fn copy_interior(&self, border: usize) -> Self
    where
        T: Clone,
    {
        assert!(border > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        assert!((2 * border) < height, "Border size must be less than half the height");
        assert!((2 * border) < width, "Border size must be less than half the width");

        let region = self.data.slice(s![border..(height - border), border..(width - border), ..]);
        Self::new(&region)
    }

    /// View a border region of the image.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than the respective axis dimension.
    #[inline]
    #[must_use]
    pub fn view_border(&self, direction: &Direction, border_size: usize) -> ArrayView3<T> {
        assert!(border_size > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        match *direction {
            Direction::North => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when viewing northern border"
                );
                self.data.slice(s![0..border_size, .., ..])
            }
            Direction::East => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when viewing eastern border"
                );
                self.data.slice(s![.., (width - border_size).., ..])
            }
            Direction::South => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when viewing southern border"
                );
                self.data.slice(s![(height - border_size).., .., ..])
            }
            Direction::West => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when viewing western border"
                );
                self.data.slice(s![.., 0..border_size, ..])
            }
        }
    }

    /// Get a mutable view of a border region of the image.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than the respective axis dimension.
    #[inline]
    #[must_use]
    pub fn view_border_mut(&mut self, direction: &Direction, border_size: usize) -> ArrayViewMut3<T> {
        assert!(border_size > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        match *direction {
            Direction::North => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when viewing northern border"
                );
                self.data.slice_mut(s![0..border_size, .., ..])
            }
            Direction::East => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when viewing eastern border"
                );
                self.data.slice_mut(s![.., (width - border_size).., ..])
            }
            Direction::South => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when viewing southern border"
                );
                self.data.slice_mut(s![(height - border_size).., .., ..])
            }
            Direction::West => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when viewing western border"
                );
                self.data.slice_mut(s![.., 0..border_size, ..])
            }
        }
    }

    /// Copy a border region of the image.
    ///
    /// # Panics
    ///
    /// Panics if the border size is not positive.
    /// Panics if the border size is larger than the respective axis dimension.
    #[inline]
    #[must_use]
    pub fn copy_border(&self, direction: &Direction, border_size: usize) -> Self
    where
        T: Clone,
    {
        assert!(border_size > 0, "Border size must be positive");
        let (height, width) = self.resolution();
        match *direction {
            Direction::North => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when copying northern border"
                );
                Self::new(&self.data.slice(s![0..border_size, .., ..]))
            }
            Direction::East => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when copying eastern border"
                );
                Self::new(&self.data.slice(s![.., (width - border_size).., ..]))
            }
            Direction::South => {
                assert!(
                    border_size <= height,
                    "Border size must be less than or equal to height when copying southern border"
                );
                Self::new(&self.data.slice(s![(height - border_size).., .., ..]))
            }
            Direction::West => {
                assert!(
                    border_size <= width,
                    "Border size must be less than or equal to width when copying western border"
                );
                Self::new(&self.data.slice(s![.., 0..border_size, ..]))
            }
        }
    }

    /// Split the image into a grid of tile views.
    ///
    /// # Panics
    ///
    /// Panics if the tile size is not positive.
    /// Panics if the overlap size is not less than the tile size.
    /// Panics if the image does not contain an integer number of tiles in either dimension.
    #[inline]
    #[must_use]
    pub fn view_tiles(&self, tile_size: (usize, usize), overlap: (usize, usize)) -> Array2<ArrayView3<T>>
    where
        T: Clone,
    {
        let (height, width) = self.resolution();
        let (tile_height, tile_width) = tile_size;
        let (overlap_height, overlap_width) = overlap;

        assert!(tile_height > 0, "Tile height must be positive");
        assert!(tile_width > 0, "Tile width must be positive");
        assert!(overlap_height < tile_height, "Overlap height must be less than tile height");
        assert!(overlap_width < tile_width, "Overlap width must be less than tile width");
        assert!(
            (height - overlap_height) % (tile_height - overlap_height) == 0,
            "Image must contain an integer number of tiles in the vertical direction"
        );
        assert!(
            (width - overlap_width) % (tile_width - overlap_width) == 0,
            "Image must contain an integer number of tiles in the horizontal direction"
        );

        let num_tiles_height = (height - overlap_height) / (tile_height - overlap_height);
        let num_tiles_width = (width - overlap_width) / (tile_width - overlap_width);

        let step_size_height = tile_height - overlap_height;
        let step_size_width = tile_width - overlap_width;

        Array2::from_shape_fn((num_tiles_height, num_tiles_width), |(row, col)| {
            let start_row = row * step_size_height;
            let start_col = col * step_size_width;
            self.view_region((start_row, start_col), tile_size)
        })
    }

    /// Split the image into a grid of tiles.
    ///
    /// # Panics
    ///
    /// Panics if the tile size is not positive.
    /// Panics if the overlap size is not less than the tile size.
    /// Panics if the image does not contain an integer number of tiles in either dimension.
    #[inline]
    #[must_use]
    pub fn copy_tiles(&self, tile_size: (usize, usize), overlap: (usize, usize)) -> Array2<Self>
    where
        T: Clone,
    {
        let (height, width) = self.resolution();
        let (tile_height, tile_width) = tile_size;
        let (overlap_height, overlap_width) = overlap;

        assert!(tile_height > 0, "Tile height must be positive");
        assert!(tile_width > 0, "Tile width must be positive");
        assert!(overlap_height < tile_height, "Overlap height must be less than tile height");
        assert!(overlap_width < tile_width, "Overlap width must be less than tile width");
        assert!(
            (height - overlap_height) % (tile_height - overlap_height) == 0,
            "Image must contain an integer number of tiles in the vertical direction"
        );
        assert!(
            (width - overlap_width) % (tile_width - overlap_width) == 0,
            "Image must contain an integer number of tiles in the horizontal direction"
        );

        let num_tiles_height = (height - overlap_height) / (tile_height - overlap_height);
        let num_tiles_width = (width - overlap_width) / (tile_width - overlap_width);

        let step_size_height = tile_height - overlap_height;
        let step_size_width = tile_width - overlap_width;

        Array2::from_shape_fn((num_tiles_height, num_tiles_width), |(row, col)| {
            let start_row = row * step_size_height;
            let start_col = col * step_size_width;
            self.copy_region((start_row, start_col), tile_size)
        })
    }

    /// Shift the pixels of the image by a given offset, wrapping around the edges toroidally.
    ///
    /// # Panics
    ///
    /// Panics if a dimension of the image exceeds `isize::MAX`.
    #[expect(
        clippy::expect_used,
        reason = "Index conversion is safe here as offset is not likely to exceed isize::MAX"
    )]
    #[inline]
    #[must_use]
    pub fn copy_slide(&self, (row_off, col_off): (isize, isize)) -> Self
    where
        T: Clone + Zero,
    {
        let (height, width) = self.resolution();
        let iheight = isize::try_from(height).expect("height exceeds isize::MAX");
        let iwidth = isize::try_from(width).expect("width exceeds isize::MAX");
        let num_channels = self.format.num_channels();

        // Negate the offsets to make positive values move in the positive direction
        let neg_row_off = -row_off;
        let neg_col_off = -col_off;

        // Calculate the actual offsets after wrapping
        let row = usize::try_from(neg_row_off.rem_euclid(iheight)).expect("Index conversion failed");
        let col = usize::try_from(neg_col_off.rem_euclid(iwidth)).expect("Index conversion failed");

        let mut out = Array3::zeros((height, width, num_channels));
        let src = self.data.view();

        out.slice_mut(s![..height - row, ..width - col, ..])
            .assign(&src.slice(s![row.., col.., ..]));
        out.slice_mut(s![..height - row, width - col.., ..])
            .assign(&src.slice(s![row.., ..col, ..]));
        out.slice_mut(s![height - row.., ..width - col, ..])
            .assign(&src.slice(s![..row, col.., ..]));
        out.slice_mut(s![height - row.., width - col.., ..])
            .assign(&src.slice(s![..row, ..col, ..]));

        Self {
            format: self.format,
            data: out,
        }
    }

    /// Shift the pixels of the image by a given offset, wrapping around the edges toroidally.
    #[inline]
    pub fn slide_inplace(&mut self, offset: (isize, isize))
    where
        T: Clone + Zero,
    {
        *self = replace(self, self.default_like()).copy_slide(offset);
    }
}

/// Get a component of a pixel from the image at the given coordinates.
impl<T> Index<(usize, usize, usize)> for Image<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize, usize)) -> &T {
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Image<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize, usize)) -> &mut T {
        &mut self.data[index]
    }
}
