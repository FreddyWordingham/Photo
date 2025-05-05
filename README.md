<p align="center" style="font-size: 2.5em">
    Photo
</p>
<p align="center">
    <img src="./assets/images/icon.png" alt="Photo Icon" width="200" style="border-radius: 5%; border: 2px solid #000;">
</p>
<p align="center" style="font-size: 1.5em">
    A lightweight, highly-generic Rust library for image manipulation with rich format support and transformation operations.
</p>

[![crates.io](https://img.shields.io/crates/v/photo.svg)](https://crates.io/crates/photo)
[![Documentation](https://docs.rs/photo/badge.svg)](https://docs.rs/photo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

Photo is a utility library for image manipulation in Rust, providing a flexible and ergonomic API for loading, manipulating, and saving images in various color formats. The library is built on top of `ndarray` and supports operations on images represented as 2D arrays of color values.

## Features

- **Generic Color Support**: Works with any color type that implements the `Colour` trait from the `chromatic` crate
- **PNG Format Support**: Load and save PNG images with various color types
- **Type-safe Image Manipulation**: Leverage Rust's type system for compile-time guarantees
- **Integration with `ndarray`**: Use the powerful n-dimensional array library for efficient image operations
- **Float-based Color Operations**: Support for floating-point color components for high-precision manipulations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
photo = "3.1.0"
```

## Quick Start

Here's a simple example of loading an image, modifying it, and saving it:

```rust
use chromatic::HsvAlpha;
use ndarray::Array2;
use photo::Image;
use std::{fs::create_dir_all, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new("input/image.png");

    // Load the image
    let mut img: Array2<HsvAlpha<f32>> = Array2::load(input_path)?;

    // Modify the image - shift hue by 60 degrees
    img.mapv_inplace(|pixel| {
        HsvAlpha::new(
            pixel.hue() + 60.0,
            pixel.saturation(),
            pixel.value(),
            pixel.alpha(),
        )
    });

    // Save the modified image
    create_dir_all("output")?;
    img.save(Path::new("output/modified.png"))?;

    Ok(())
}
```

## Working with Different Color Types

Photo supports various color types through the `chromatic` crate:

```rust
// RGB colors
use chromatic::Rgb;
let img: Array2<Rgb<f32>> = Array2::load("rgb_image.png")?;

// RGBA colors
use chromatic::RgbAlpha;
let img: Array2<RgbAlpha<f32>> = Array2::load("rgba_image.png")?;

// HSV colors
use chromatic::Hsv;
let img: Array2<Hsv<f32>> = Array2::load("hsv_image.png")?;

// HSV with alpha
use chromatic::HsvAlpha;
let img: Array2<HsvAlpha<f32>> = Array2::load("hsva_image.png")?;

// Grayscale
use chromatic::Gray;
let img: Array2<Gray<f32>> = Array2::load("gray_image.png")?;
```

## Advanced Usage

### Image Transformations

Leverage `ndarray`'s powerful operations for image manipulation:

```rust
use ndarray::s;

// Extract a region of the image
let region = img.slice(s![100..300, 200..400]);

// Flip the image horizontally
let flipped = img.slice(s![.., ..;-1]);

// Rotate the image 90 degrees
let rotated = img.t();
```

### Pixel-wise Operations

Apply transformations to each pixel:

```rust
// Invert an RGB image
use chromatic::Rgb;
let inverted = rgb_image.mapv(|px: Rgb<f32>| {
    Rgb::new(
        1.0 - px.red(),
        1.0 - px.green(),
        1.0 - px.blue()
    )
});

// Adjust brightness
let brightened = img.mapv(|px: HsvAlpha<f32>| {
    HsvAlpha::new(
        px.hue(),
        px.saturation(),
        (px.value() * 1.2).min(1.0), // Increase brightness by 20%
        px.alpha()
    )
});
```

## Error Handling

The library provides a comprehensive error type `PngError` that covers various failure modes:

- I/O errors
- PNG encoding/decoding errors
- Unsupported color types
- Unsupported bit depths
- Invalid channel counts
- Invalid data

## Dependencies

- `chromatic`: Color manipulation library
- `ndarray`: N-dimensional array manipulation
- `num-traits`: Numeric trait abstractions
- `png`: PNG format encoding/decoding
- `vista`: (dev dependency) for visualization in the terminal
