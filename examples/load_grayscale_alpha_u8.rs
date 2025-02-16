use ndarray_images::ImageGA;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "grayscale_alpha-u8.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageGA::<u8>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
