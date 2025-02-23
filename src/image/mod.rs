use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis, s};
use num_traits::Zero;
use std::{collections::HashMap, hash::Hash};

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

    /// Create a new Image from a mapping.
    pub fn new_from_mapping(tile_mapping: &Array2<usize>, unique_tiles: &[Image<T>]) -> Self {
        debug_assert!(tile_mapping.dim().0 > 0);
        debug_assert!(tile_mapping.dim().1 > 0);
        debug_assert!(tile_mapping.iter().all(|&index| index < unique_tiles.len()));

        let tile_size = unique_tiles[0].data.dim();
        let height = tile_mapping.dim().0 * tile_size.0;
        let width = tile_mapping.dim().1 * tile_size.1;
        let mut data = Array2::zeros((height, width));

        for (index, &tile_index) in tile_mapping.iter().enumerate() {
            let tile = &unique_tiles[tile_index];
            let row = index / tile_mapping.dim().1;
            let col = index % tile_mapping.dim().1;
            let start = [row * tile_size.0, col * tile_size.1];
            let end = [start[0] + tile_size.0, start[1] + tile_size.1];
            data.slice_mut(s![start[0]..end[0], start[1]..end[1]])
                .assign(&tile.data);
        }

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

    /// Transposes the image.
    pub fn transpose(&mut self) {
        self.data = self.data.t().to_owned();
    }

    /// Flips the image vertically.
    pub fn flip_vertical(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    /// Flips the image horizontally.
    pub fn flip_horizontal(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    /// Rotates the image 90 degrees clockwise (right).
    pub fn rotate_clockwise(&mut self) {
        self.data = self.data.t().slice(s![.., ..;-1]).to_owned();
    }

    /// Rotates the image 90 degrees anticlockwise (left).
    pub fn rotate_anticlockwise(&mut self) {
        self.data = self.data.t().slice(s![..;-1, ..]).to_owned();
    }

    /// Rotates the image 180 degrees.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
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

    /// Extract a tile from the image.
    pub fn extract_tile(&self, tile_size: [usize; 2], tile_index: [usize; 2]) -> Image<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);
        self.extract(
            [tile_index[0] * tile_size[0], tile_index[1] * tile_size[1]],
            tile_size,
        )
    }

    /// Create a view to a tile of the image.
    pub fn view_tile(&self, tile_size: [usize; 2], tile_index: [usize; 2]) -> ArrayView2<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);
        self.data.slice(s![
            tile_index[0] * tile_size[0]..(tile_index[0] + 1) * tile_size[0],
            tile_index[1] * tile_size[1]..(tile_index[1] + 1) * tile_size[1]
        ])
    }

    /// Create a mutable view to a tile of the image.
    pub fn view_tile_mut(
        &mut self,
        tile_size: [usize; 2],
        tile_index: [usize; 2],
    ) -> ArrayViewMut2<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);

        self.data.slice_mut(s![
            tile_index[0] * tile_size[0]..(tile_index[0] + 1) * tile_size[0],
            tile_index[1] * tile_size[1]..(tile_index[1] + 1) * tile_size[1]
        ])
    }

    /// Split the image into equal-sized tiles.
    pub fn tiles(&self, tile_size: [usize; 2]) -> Array2<Image<T>> {
        let height = self.height();
        let width = self.width();

        debug_assert!(height % tile_size[0] == 0);
        debug_assert!(width % tile_size[1] == 0);

        let tile_rows = height / tile_size[0];
        let tile_cols = width / tile_size[1];

        Array2::from_shape_fn((tile_rows, tile_cols), |(row, col)| {
            let y = row * tile_size[0];
            let x = col * tile_size[1];
            let tile = self
                .data
                .slice(s![y..y + tile_size[0], x..x + tile_size[1]])
                .to_owned();
            Image { data: tile }
        })
    }
}

impl<T: Default + Copy + PartialOrd + Zero + Eq + Hash> Image<T> {
    /// Create a list of all unique tiles in the image and their frequency.
    pub fn unique_tiles(&self, tile_size: [usize; 2]) -> Vec<(Image<T>, usize)> {
        let tiles = self.tiles(tile_size);
        let mut freq_map: HashMap<Vec<T>, (Image<T>, usize)> = HashMap::new();

        for tile in tiles.iter() {
            let key: Vec<T> = tile.data.iter().copied().collect();
            freq_map
                .entry(key)
                .and_modify(|(_, count)| *count += 1)
                .or_insert((tile.clone(), 1));
        }

        freq_map.into_values().collect()
    }
}

mod lin_srgb;
mod lin_srgba;
