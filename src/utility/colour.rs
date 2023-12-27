//! Colour utility functions.

use palette::LinSrgba;

/// Convert a 32-bit RGBA colour to a linear sRGBA colour.
#[must_use]
#[inline]
pub fn from_u32(colour: u32) -> LinSrgba {
    let red = ((colour >> 24) & 0xFF) as f32 / 255.0;
    let green = ((colour >> 16) & 0xFF) as f32 / 255.0;
    let blue = ((colour >> 8) & 0xFF) as f32 / 255.0;
    let alpha = (colour & 0xFF) as f32 / 255.0;

    LinSrgba::new(red, green, blue, alpha)
}
