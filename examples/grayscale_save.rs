use ndarray::Array2;
use ndarray_images::Image;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "grayscale.png";

fn main() {
    let width = 20;
    let height = 10;
    let mut image = Array2::zeros((height, width));

    for i in 0..height {
        image[[i, i]] = 1.0;
    }

    println!("{:?}", image);

    let image_path = &format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);
    image.save(image_path).expect("Failed to save image");
}
