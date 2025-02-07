use ndarray::Array2;
use ndarray_images::Image;

fn main() {
    let width = 20;
    let height = 10;
    let mut image = Array2::zeros((height, width));

    for i in 0..height {
        image[[i, i]] = 1.0;
    }

    image
        .save("output/image.png")
        .expect("Failed to save image");
}
