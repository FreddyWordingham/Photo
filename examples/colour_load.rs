use ndarray::Array3;
use ndarray_images::Image;

const INPUT_DIR: &str = "input";
const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "colour.png";

fn main() {
    let image_path = &format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image: Array3<f32> = Array3::load(image_path).expect("Failed to load image");

    println!("{:?}", image);

    let output_path = &format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);
    image.save(output_path).expect("Failed to save image");
}
