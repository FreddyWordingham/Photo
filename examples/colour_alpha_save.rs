use ndarray::{arr1, s, Array3};
use ndarray_images::Image;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "colour_alpha.png";

fn main() {
    let width = 20;
    let height = 10;
    let mut image = Array3::zeros((height, width, 4));

    // Set the red and alpha channels to 1.0 for all pixels
    image.slice_mut(s![.., .., 0]).fill(1.0);
    image.slice_mut(s![.., .., 3]).fill(1.0);

    // Draw transparent vertical lines every 3 pixels
    image.slice_mut(s![.., 1..;3, 3]).fill(0.0);

    // Set the make the pixels solid blue along the diagonal
    for i in 0..height {
        image
            .slice_mut(s![i, i, ..])
            .assign(&arr1(&[0.0, 0.0, 1.0, 1.0]));
    }

    let image_path = &format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);
    image.save(image_path).expect("Failed to save image");
}
