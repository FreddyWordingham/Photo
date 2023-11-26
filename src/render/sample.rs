use palette::LinSrgba;
use std::ops::{AddAssign, DivAssign};

#[derive(Debug, Clone)]
pub struct Sample {
    pub pixel_index: [usize; 2],
    pub colour: LinSrgba,
}

impl Sample {
    pub fn new(pixel_index: [usize; 2], colour: LinSrgba) -> Self {
        Self {
            pixel_index,
            colour,
        }
    }
}

impl AddAssign for Sample {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert_eq!(self.pixel_index, rhs.pixel_index);

        self.colour += rhs.colour;
    }
}

impl DivAssign<f32> for Sample {
    fn div_assign(&mut self, rhs: f32) {
        self.colour.red /= rhs;
        self.colour.green /= rhs;
        self.colour.blue /= rhs;
        self.colour.alpha /= rhs;

        debug_assert!(self.colour.red >= 0.0);
        debug_assert!(self.colour.green >= 0.0);
        debug_assert!(self.colour.blue >= 0.0);
        debug_assert!(self.colour.alpha >= 0.0);
        debug_assert!(self.colour.red <= 1.0);
        debug_assert!(self.colour.green <= 1.0);
        debug_assert!(self.colour.blue <= 1.0);
        debug_assert!(self.colour.alpha <= 1.0);
    }
}
