use enterpolation::Merge;
use ndarray::{s, Array2, Axis};
use num_traits::{Float, FromPrimitive, Zero};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use crate::{colour_map::ColorFromHex, ColourMap, Image};

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
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let data = Array2::zeros((height, width));
        Self { data }
    }

    /// Creates an image filled with a constant value.
    pub fn filled(width: usize, height: usize, value: [T; 1]) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let data = Array2::from_elem((height, width), value[0]);
        Self { data }
    }

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    /// Gets the value of a component (the only one) at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 1);
        self.data[[coords[1], coords[0]]]
    }

    /// Sets the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 1);
        self.data[[coords[1], coords[0]]] = value;
    }

    /// Gets the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 1] {
        [self.data[[coords[1], coords[0]]]]
    }

    /// Sets the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 1]) {
        self.data[[coords[1], coords[0]]] = pixel[0];
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

    /// Rotates the image 90° clockwise.
    pub fn rotate_clockwise(&mut self) {
        self.data = self.data.t().slice(s![.., ..;-1]).to_owned();
    }

    /// Rotates the image 90° anticlockwise.
    pub fn rotate_anticlockwise(&mut self) {
        self.data = self.data.t().slice(s![..;-1, ..]).to_owned();
    }

    /// Rotates the image 180°.
    pub fn rotate_180(&mut self) {
        self.data.invert_axis(Axis(0));
        self.data.invert_axis(Axis(1));
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
    pub fn extract(&self, x: usize, y: usize, width: usize, height: usize) -> ImageG<T> {
        Self::new(self.data.slice(s![y..y + height, x..x + width]).to_owned())
    }

    // /// Split the image into equal-sized tiles.
    // pub fn tiles(self, tile_width: usize, tile_height: usize) -> Vec<ImageG<T>> {
    //     let mut tiles = Vec::new();
    //     for y in (0..self.height()).step_by(tile_height) {
    //         for x in (0..self.width()).step_by(tile_width) {
    //             let tile = self
    //                 .data
    //                 .slice(s![y..y + tile_height, x..x + tile_width])
    //                 .to_owned();
    //             tiles.push(ImageG { data: tile });
    //         }
    //     }
    //     tiles
    // }
}

mod float;
mod u8;
