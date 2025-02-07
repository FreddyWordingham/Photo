use ndarray::{arr1, s, Array3};
use ndarray_images::Image;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "colour.png";

fn main() {
    let width = 20;
    let height = 10;
    let mut image = Array3::zeros((height, width, 3));

    // Set the red channel (first layer) to 1.0 for all pixels
    image.slice_mut(s![.., .., 0]).fill(1.0);

    for i in 0..height {
        image
            .slice_mut(s![i, i, ..])
            .assign(&arr1(&[0.0, 0.0, 1.0]));
    }

    let image_path = &format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);
    image.save(image_path).expect("Failed to save image");
}
