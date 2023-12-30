//! Single ray sampling structure.

use core::{
    ops::{AddAssign, MulAssign},
    time::Duration,
};

use palette::LinSrgba;

/// Scene sample.
#[derive(Clone)]
#[non_exhaustive]
pub struct Sample {
    /// Index of the pixel in the image [row, column].
    pub pixel_index: [usize; 2],
    /// Total colour.
    pub colour: LinSrgba,
    /// Total time.
    pub time: Duration,
}

impl Sample {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pixel_index: [usize; 2], colour: LinSrgba, time: Duration) -> Self {
        Self {
            pixel_index,
            colour,
            time,
        }
    }
}

impl AddAssign for Sample {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        debug_assert_eq!(
            self.pixel_index, rhs.pixel_index,
            "Pixel indices must match."
        );

        self.colour += rhs.colour;
    }
}

impl MulAssign<f32> for Sample {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        debug_assert!(rhs.is_finite(), "Sample multiplier must be finite.");

        self.colour.red *= rhs;
        self.colour.green *= rhs;
        self.colour.blue *= rhs;
        self.colour.alpha *= rhs;

        debug_assert!(
            (0.0..=1.0).contains(&self.colour.red),
            "Colour red channel value must be in the range [0.0, 1.0]."
        );
        debug_assert!(
            (0.0..=1.0).contains(&self.colour.red),
            "Colour green channel value must be in the range [0.0, 1.0]."
        );
        debug_assert!(
            (0.0..=1.0).contains(&self.colour.red),
            "Colour blue channel value must be in the range [0.0, 1.0]."
        );
        debug_assert!(
            (0.0..=1.0).contains(&self.colour.red),
            "Colour alpha channel value must be in the range [0.0, 1.0]."
        );
    }
}
