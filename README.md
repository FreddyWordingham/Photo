# Image

(De)Serialize NDarray to/from PNG files.

## Quick Start

### Grayscale Images

#### Load

Greyscale images can be loaded into a floating point NDarray with values in the range [0, 1]:

```rust
let image: Array2<f32> = Array2::load("image.png")?;
```

#### Save

2D NDarrays can be saved as greyscale images:

```rust
image.save("image.png")?;
```

### Colour Images

#### Load

Similarly, colour images can be loaded into a 3D NDarray with values in the range [0, 1].
If the image has transparency (an alpha channel) there will be 4 channels, otherwise there will be 3:

```rust
let image: Array3<f32> = Array3::load("image.png")?;
```

#### Save

Colour images can be saved in the same way:

```rust
image.save("image.png")?;
```
