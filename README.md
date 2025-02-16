# Photo

Utility image classes for Rust.

## Features

- Grayscale, Grascale with alpha, RGB and RGBA images, as well as custom color types.
- Colour maps
- Image IO
- Image transformations utilities.
- Generic precision types for image components: `u8`, `f32`, `f64` etc.
- Print images to the console.

## Usage

### Reading and writing images

Loading a RGB image with f32 components:

```rust
use photo::ImageRGB;

let mut image = ImageRGB::<f32>::load("input/my_colour_image.png").expect("Failed to load image");
```

Save it:

```rust
image.save("output/my_colour_image.png").expect("Failed to save image");
```

### Image transformations

```rust
image.flip_horizontal();
image.rotate_clockwise();
```

### Colour maps

```rust
let colours = vec!["#FF0000", "#00FF00", "#0000FF00"];
let colour_map: ColourMap<f32, LinSrgba> = ColourMap::new(&colours);

let sample = colour_map.sample(0.75);
```

### Colourize grayscale images

```rust
use photo::ImageG;

let grayscale_image = ImageG::<u8>::load("input/my_grayscale_image.png").expect("Failed to load image");
let coloured_image = grayscale_image.colourize(&colour_map);
```

### Print images to the console

```rust
println!("{}", image);
```
