use ndarray::{arr1, s, stack, Array2, Array3, Axis};
use num_traits::Zero;
use std::{collections::HashMap, hash::Hash};

/// A grayscale image with transparency.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageGA<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

impl<T: Copy + PartialOrd + Zero> ImageGA<T> {
    /// Creates a new ImageGA from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0 && data.dim().1 > 0);
        debug_assert!(data.dim().2 == 2);
        Self { data }
    }

    /// Creates an empty image (all zeros) with alpha set to one.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0 && height > 0);
        let data = Array3::zeros((height, width, 2));
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(width: usize, height: usize, value: [T; 2]) -> Self {
        debug_assert!(width > 0 && height > 0);
        let mut data = Array3::zeros((height, width, 2));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        Self { data }
    }

    /// Creates an ImageGA from two grayscale layers.
    pub fn from_layers(layers: [Array2<T>; 2]) -> Self {
        debug_assert!(layers.iter().all(|layer| layer.ncols() > 0));
        debug_assert!(layers.iter().all(|layer| layer.nrows() > 0));
        debug_assert!(layers.iter().all(|layer| layer.dim() == layers[0].dim()));
        let data =
            stack(Axis(2), &[layers[0].view(), layers[1].view()]).expect("Failed to stack layers");
        Self { data }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Gets the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 2);
        self.data[[coords[1], coords[0], component]]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 2);
        self.data[[coords[1], coords[0], component]] = value;
    }

    /// Gets the pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 2] {
        let pixel_slice = self.data.slice(s![coords[1], coords[0], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Sets the pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 2]) {
        let mut view = self.data.slice_mut(s![coords[1], coords[0], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Gets a component layer of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
        debug_assert!(component < 2);
        self.data.slice(s![.., .., component]).to_owned()
    }

    /// Transposes the image.
    pub fn transpose(&mut self) {
        self.data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
    }

    /// Flips the image vertically.
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    /// Flips the image horizontally.
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Rotates the image 90 degrees clockwise.
    pub fn rotate_clockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(1));
        self.data = new_data;
    }

    /// Rotates the image 90 degrees anticlockwise.
    pub fn rotate_anticlockwise(&mut self) {
        let mut new_data = self.data.clone().permuted_axes([1, 0, 2]).to_owned();
        new_data.invert_axis(Axis(0));
        self.data = new_data;
    }

    /// Rotates the image 180 degrees.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
    }

    /// Extract a portion of the image.
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> ImageGA<T> {
        debug_assert!(start[0] + size[0] <= self.width());
        debug_assert!(start[1] + size[1] <= self.height());
        debug_assert!(size.iter().all(|&s| s > 0));
        Self::new(
            self.data
                .slice(s![
                    start[1]..start[1] + size[1],
                    start[0]..start[0] + size[0],
                    ..
                ])
                .to_owned(),
        )
    }

    /// Extract a tile from the image.
    pub fn extract_tile(&self, tile_size: [usize; 2], tile_index: [usize; 2]) -> ImageGA<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.width() / tile_size[0]);
        debug_assert!(tile_index[1] < self.height() / tile_size[1]);
        self.extract(
            [tile_index[0] * tile_size[0], tile_index[1] * tile_size[1]],
            tile_size,
        )
    }

    /// Split the image into equal-sized tiles.
    pub fn tiles(&self, tile_size: [usize; 2]) -> Array2<ImageGA<T>> {
        let width = self.width();
        let height = self.height();

        debug_assert!(width % tile_size[0] == 0);
        debug_assert!(height % tile_size[1] == 0);

        let tile_rows = height / tile_size[1];
        let tile_cols = width / tile_size[0];

        Array2::from_shape_fn((tile_rows, tile_cols), |(row, col)| {
            let y = row * tile_size[1];
            let x = col * tile_size[0];
            let tile = self
                .data
                .slice(s![y..y + tile_size[1], x..x + tile_size[0], ..])
                .to_owned();
            ImageGA { data: tile }
        })
    }
}

impl<T: Copy + PartialOrd + Zero + Eq + Hash> ImageGA<T> {
    /// Create a list of all unique tiles in the image and their frequency.
    pub fn unique_tiles(&self, tile_size: [usize; 2]) -> Vec<(ImageGA<T>, usize)> {
        let tiles = self.tiles(tile_size);
        let mut freq_map: HashMap<Vec<T>, (ImageGA<T>, usize)> = HashMap::new();

        for tile in tiles.iter() {
            let key: Vec<T> = tile.data.iter().copied().collect();
            freq_map
                .entry(key)
                .and_modify(|(_, count)| *count += 1)
                .or_insert((tile.clone(), 1));
        }

        freq_map.into_iter().map(|(_, v)| v).collect()
    }
}

mod float;
mod u8;
