# Image

(De)Serialize NDarray to/from PNG files.

## Quick Start

### Load an image into a NDarray

Greyscale images can be loaded into a floating point NDarray with values in the range [0, 1]:

```rust
use ndarray::Array2;
use image::Image;

fn main() {
    let image = Image::open("image.png");
}
```
