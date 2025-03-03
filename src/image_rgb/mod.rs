use indexmap::IndexMap;
use ndarray::{Array2, Array3, ArrayView3, ArrayViewMut3, Axis, arr1, s, stack};
use num_traits::Zero;
use std::hash::Hash;

use crate::Transformation;

/// An opaque colour image.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageRGB<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

impl<T: Copy + PartialOrd + Zero> ImageRGB<T> {
    /// Creates a new ImageRGB from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 3);
        Self { data }
    }

    /// Create a new ImageRGB from a mapping.
    pub fn new_from_mapping(tile_mapping: &Array2<usize>, unique_tiles: &[ImageRGB<T>]) -> Self {
        debug_assert!(tile_mapping.dim().0 > 0);
        debug_assert!(tile_mapping.dim().1 > 0);
        debug_assert!(tile_mapping.iter().all(|&index| index < unique_tiles.len()));

        let tile_size = unique_tiles[0].data.dim();
        let height = tile_mapping.dim().0 * tile_size.0;
        let width = tile_mapping.dim().1 * tile_size.1;
        let mut data = Array3::zeros((height, width, 3));

        for (index, &tile_index) in tile_mapping.iter().enumerate() {
            let tile = &unique_tiles[tile_index];
            let row = index / tile_mapping.dim().1;
            let col = index % tile_mapping.dim().1;
            let start = [row * tile_size.0, col * tile_size.1];
            let end = [start[0] + tile_size.0, start[1] + tile_size.1];
            data.slice_mut(s![start[0]..end[0], start[1]..end[1], ..])
                .assign(&tile.data);
        }

        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(resolution: [usize; 2]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let data = Array3::zeros((resolution[0], resolution[1], 3));
        Self { data }
    }

    /// Creates an image filled with a constant RGB value.
    pub fn filled(resolution: [usize; 2], value: [T; 3]) -> Self {
        debug_assert!(resolution.iter().all(|&r| r > 0));
        let mut data = Array3::zeros((resolution[0], resolution[1], 3));
        data.slice_mut(s![.., .., 0]).fill(value[0]);
        data.slice_mut(s![.., .., 1]).fill(value[1]);
        data.slice_mut(s![.., .., 2]).fill(value[2]);
        Self { data }
    }

    /// Creates an ImageRGB from three color layers.
    pub fn from_layers(layers: [Array2<T>; 3]) -> Self {
        debug_assert!(layers.iter().all(|layer| layer.ncols() > 0));
        debug_assert!(layers.iter().all(|layer| layer.nrows() > 0));
        debug_assert!(layers.iter().all(|layer| layer.dim() == layers[0].dim()));
        let data = stack(
            Axis(2),
            &[layers[0].view(), layers[1].view(), layers[2].view()],
        )
        .expect("Failed to stack layers");
        Self { data }
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
        debug_assert!(component < 3);
        self.data[[coords[0], coords[1], component]]
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 3);
        self.data[[coords[0], coords[1], component]] = value;
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 3] {
        let pixel_slice = self.data.slice(s![coords[0], coords[1], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice is not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 3]) {
        let mut view = self.data.slice_mut(s![coords[0], coords[1], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Get a color channel of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
        debug_assert!(component < 3);
        self.data.slice(s![.., .., component]).to_owned()
    }

    /// Apply a transformation to the image.
    pub fn transform(&mut self, transform: Transformation) {
        self.data = match transform {
            Transformation::Identity => self.data.clone(),
            Transformation::Rotate90 => self
                .data
                .clone()
                .permuted_axes([1, 0, 2])
                .slice(s![.., ..;-1, ..])
                .to_owned(),
            Transformation::Rotate180 => self.data.slice(s![..;-1, ..;-1, ..]).to_owned(),
            Transformation::Rotate270 => self
                .data
                .clone()
                .permuted_axes([1, 0, 2])
                .slice(s![..;-1, .., ..])
                .to_owned(),
            Transformation::FlipHorizontal => self.data.slice(s![.., ..;-1, ..]).to_owned(),
            Transformation::FlipVertical => self.data.slice(s![..;-1, .., ..]).to_owned(),
            Transformation::FlipDiagonal => self.data.clone().permuted_axes([1, 0, 2]).to_owned(),
            Transformation::FlipAntiDiagonal => self
                .data
                .slice(s![..;-1, ..;-1, ..])
                .to_owned()
                .permuted_axes([1, 0, 2])
                .to_owned(),
        };
    }

    /// Extract a portion of the image.
    pub fn extract(&self, start: [usize; 2], size: [usize; 2]) -> ImageRGB<T> {
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

    /// Extract a tile from the image.
    pub fn extract_tile(&self, tile_size: [usize; 2], tile_index: [usize; 2]) -> ImageRGB<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);
        self.extract(
            [tile_index[0] * tile_size[0], tile_index[1] * tile_size[1]],
            tile_size,
        )
    }

    /// Create a view to a tile of the image.
    pub fn view_tile(&self, tile_size: [usize; 2], tile_index: [usize; 2]) -> ArrayView3<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);
        self.data.slice(s![
            tile_index[0] * tile_size[0]..(tile_index[0] + 1) * tile_size[0],
            tile_index[1] * tile_size[1]..(tile_index[1] + 1) * tile_size[1],
            ..
        ])
    }

    /// Create a mutable view to a tile of the image.
    pub fn view_tile_mut(
        &mut self,
        tile_size: [usize; 2],
        tile_index: [usize; 2],
    ) -> ArrayViewMut3<T> {
        debug_assert!(tile_size.iter().all(|&s| s > 0));
        debug_assert!(tile_index[0] < self.height() / tile_size[0]);
        debug_assert!(tile_index[1] < self.width() / tile_size[1]);

        self.data.slice_mut(s![
            tile_index[0] * tile_size[0]..(tile_index[0] + 1) * tile_size[0],
            tile_index[1] * tile_size[1]..(tile_index[1] + 1) * tile_size[1],
            ..
        ])
    }

    /// Split the image into equal-sized tiles.
    pub fn tiles(&self, tile_size: [usize; 2]) -> Array2<ImageRGB<T>> {
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
                .slice(s![y..y + tile_size[0], x..x + tile_size[1], ..])
                .to_owned();
            ImageRGB { data: tile }
        })
    }
}

impl<T: Copy + PartialOrd + Zero + Eq + Hash> ImageRGB<T> {
    /// Create a list of all unique tiles in the image and their frequency.
    pub fn unique_tiles(&self, tile_size: [usize; 2]) -> Vec<(ImageRGB<T>, usize)> {
        let tiles = self.tiles(tile_size);
        let mut freq_map: IndexMap<Vec<T>, (ImageRGB<T>, usize)> = IndexMap::new();

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

mod float;
mod u8;
