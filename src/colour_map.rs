use enterpolation::{linear::Linear, Generator, Identity, Merge, Sorted};
use num_traits::{Float, FromPrimitive};
use palette::{LinSrgb, LinSrgba};
use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub trait ColorFromHex<T>
where
    T: Float + FromPrimitive,
{
    fn from_hex(hex: &str) -> Self;
}

impl<T> ColorFromHex<T> for LinSrgba<T>
where
    T: Float + FromPrimitive,
{
    fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).expect(&format!("Invalid hex code: {}", hex));
        let g = u8::from_str_radix(&hex[2..4], 16).expect(&format!("Invalid hex code: {}", hex));
        let b = u8::from_str_radix(&hex[4..6], 16).expect(&format!("Invalid hex code: {}", hex));
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).expect(&format!("Invalid hex code: {}", hex))
        } else {
            255
        };

        let from_255 = T::from_u8(255).expect("Conversion failed");
        let r = T::from_u8(r).expect("Conversion failed") / from_255;
        let g = T::from_u8(g).expect("Conversion failed") / from_255;
        let b = T::from_u8(b).expect("Conversion failed") / from_255;
        let a = T::from_u8(a).expect("Conversion failed") / from_255;
        LinSrgba::new(r, g, b, a)
    }
}

impl<T> ColorFromHex<T> for LinSrgb<T>
where
    T: Float + FromPrimitive,
{
    fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).expect(&format!("Invalid hex code: {}", hex));
        let g = u8::from_str_radix(&hex[2..4], 16).expect(&format!("Invalid hex code: {}", hex));
        let b = u8::from_str_radix(&hex[4..6], 16).expect(&format!("Invalid hex code: {}", hex));
        let from_255 = T::from_u8(255).expect("Conversion failed");
        let r = T::from_u8(r).expect("Conversion failed") / from_255;
        let g = T::from_u8(g).expect("Conversion failed") / from_255;
        let b = T::from_u8(b).expect("Conversion failed") / from_255;
        LinSrgb::new(r, g, b)
    }
}

/// Generic colour map, parameterized over a colour type `C`.
pub struct ColourMap<T, C> {
    gradient: Linear<Sorted<Vec<T>>, Vec<C>, Identity>,
}

impl<T, C> ColourMap<T, C>
where
    T: Float + FromPrimitive + Debug,
    C: ColorFromHex<T>
        + Debug
        + Copy
        + Add<Output = C>
        + Sub<Output = C>
        + Mul<T, Output = C>
        + Div<T, Output = C>
        + Merge<T>,
{
    pub fn new(colour_hexes: &[&str]) -> Self {
        assert!(!colour_hexes.is_empty(), "No colours provided");
        let colours: Vec<C> = colour_hexes.iter().map(|&hex| C::from_hex(hex)).collect();
        let num_colours = colours.len();
        let gradient = Linear::builder()
            .elements(colours)
            .knots(linspace::<T>(num_colours))
            .build()
            .expect("Failed to build gradient.");

        Self { gradient }
    }

    pub fn sample(&self, t: T) -> C {
        debug_assert!(t >= T::zero() && t <= T::one());
        <Linear<Sorted<Vec<T>>, Vec<C>, Identity> as Generator<T>>::gen(&self.gradient, t)
    }
}

fn linspace<T>(n: usize) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    assert!(n >= 2, "n must be at least 2");
    let n_minus_one = T::from_usize(n - 1).expect("Conversion failed");
    let step = T::one() / n_minus_one;
    (0..n)
        .map(|i| T::from_usize(i).expect("Conversion failed") * step)
        .collect()
}
