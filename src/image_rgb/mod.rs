use ndarray::{arr1, s, stack, Array2, Array3, Axis};
use num_traits::{One, Zero};

/// An opaque colour image.
#[derive(Debug, Clone, PartialEq)]
pub struct ImageRGB<T> {
    /// Image data stored in row-major order.
    pub data: Array3<T>,
}

impl<T: Copy + PartialOrd + Zero + One> ImageRGB<T> {
    /// Creates a new ImageRGB from the provided data.
    pub fn new(data: Array3<T>) -> Self {
        debug_assert!(data.dim().0 > 0);
        debug_assert!(data.dim().1 > 0);
        debug_assert!(data.dim().2 == 3);
        Self { data }
    }

    /// Creates an empty (all zeros) image with the given dimensions.
    pub fn empty(width: usize, height: usize) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let data = Array3::zeros((height, width, 3));
        Self { data }
    }

    /// Creates an image filled with a constant RGB value.
    pub fn filled(width: usize, height: usize, value: [T; 3]) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);
        let mut data = Array3::zeros((height, width, 3));
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

    /// Returns the width of the image.
    pub fn width(&self) -> usize {
        self.data.dim().1
    }

    /// Returns the height of the image.
    pub fn height(&self) -> usize {
        self.data.dim().0
    }

    /// Get the value of a component at the specified position.
    pub fn get_component(&self, coords: [usize; 2], component: usize) -> T {
        debug_assert!(component < 3);
        self.data[[coords[1], coords[0], component]]
    }

    /// Set the value of a component at the specified position.
    pub fn set_component(&mut self, coords: [usize; 2], component: usize, value: T) {
        debug_assert!(component < 3);
        self.data[[coords[1], coords[0], component]] = value;
    }

    /// Get the value of a pixel at the specified position.
    pub fn get_pixel(&self, coords: [usize; 2]) -> [T; 3] {
        let pixel_slice = self.data.slice(s![coords[1], coords[0], ..]);
        pixel_slice
            .as_slice()
            .expect("Pixel slice is not contiguous")
            .try_into()
            .expect("Slice length mismatch")
    }

    /// Set the value of a pixel at the specified position.
    pub fn set_pixel(&mut self, coords: [usize; 2], pixel: [T; 3]) {
        let mut view = self.data.slice_mut(s![coords[1], coords[0], ..]);
        view.assign(&arr1(&pixel));
    }

    /// Get a color channel of the image.
    pub fn get_layer(&self, component: usize) -> Array2<T> {
        debug_assert!(component < 3);
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
}

mod float;
mod u8;
