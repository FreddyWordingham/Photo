use ndarray::{Array3, ArrayBase, ArrayView1, ArrayView2, Axis, Data, Ix1, Ix2, Ix3, s, stack};
use num_traits::Zero;
use std::ops::{Index, IndexMut};

use crate::Channels;

/// Representation of an image.
#[derive(Debug, Clone, PartialEq)]
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
    #[must_use]
    pub fn new<S>(data: ArrayBase<S, Ix3>) -> Self
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
    #[must_use]
    pub fn filled((height, width): (usize, usize), values: &[T]) -> Self
    where
        T: Zero + Clone,
    {
        assert!(height > 0, "Height must be positive");
        assert!(width > 0, "Width must be positive");
        let format = Channels::from_num_channels(values.len()).expect("Number of channels must be between 1 and 4");

        let mut data = Array3::zeros((height, width, format.num_channels()));
        for (c, v) in values.iter().enumerate().take(format.num_channels()) {
            data.slice_mut(s![.., .., c]).fill(v.clone());
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
        let views: Vec<_> = layers.iter().map(|layer| layer.view()).collect();

        // Stack the views along the channel axis
        let data = stack(Axis(2), &views).expect("Failed to stack layers");

        Self { format, data }
    }

    /// Creates an `Image` from a 2D grid of tiles.
    ///
    /// # Panics
    ///
    /// Panics if the grid dimensions are not positive.
    /// Panics if the tiles do not all have the same dimensions.
    /// Panics if the tile width or height is not positive.
    /// Panics if the tiles do not all have the same format.
    pub fn from_tiles<D>(tiles: &ArrayBase<D, Ix2>) -> Self
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

    /// Returns the height of the image.
    #[must_use]
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Returns the width of the image.
    #[must_use]
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Returns the size of the image as (height, width).
    #[must_use]
    pub fn resolution(&self) -> (usize, usize) {
        (self.data.dim().0, self.data.dim().1)
    }

    /// Returns the format of the image.
    #[must_use]
    pub fn format(&self) -> Channels {
        self.format
    }

    /// Get a layer of the image as a 2D view.
    pub fn get_channel(&self, channel: usize) -> ArrayView2<T> {
        assert!(channel < self.data.dim().2, "Channel index out of bounds");
        self.data.slice(s![.., .., channel])
    }

    /// Set a layer of the image.
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
    pub fn get_pixel(&self, (row, col): (usize, usize)) -> ArrayView1<T> {
        let (height, width, _) = self.data.dim();
        assert!(row < height && col < width, "Pixel index out of bounds");
        self.data.slice(s![row, col, ..])
    }

    /// Set a pixel at the given coordinates, accepting any type that can be viewed as Array1
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
}

/// Get a component of a pixel from the image at the given coordinates.
impl<T> Index<(usize, usize, usize)> for Image<T> {
    type Output = T;

    fn index(&self, (row, col, channel): (usize, usize, usize)) -> &T {
        &self.data[[row, col, channel]]
    }
}

impl<T> IndexMut<(usize, usize, usize)> for Image<T> {
    fn index_mut(&mut self, (row, col, channel): (usize, usize, usize)) -> &mut T {
        &mut self.data[[row, col, channel]]
    }
}
