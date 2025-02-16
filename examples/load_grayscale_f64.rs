use ndarray_images::ImageG;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "grayscale-f64.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageG::<f64>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
