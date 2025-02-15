use ndarray_images::ImageGA;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "grayscale_alpha.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageGA::<f32>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
