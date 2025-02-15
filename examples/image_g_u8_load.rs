use ndarray_images::ImageG;

const INPUT_DIR: &str = "input";
const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "grayscale.png";

fn main() {
    let image_path = &format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let mut image = ImageG::<u8>::load(image_path).expect("Failed to load image");

    println!("{}", image);
    image.flip_horizontal();
    println!("{}", image);
    image.rotate_clockwise();
    println!("{}", image);

    let output_path = &format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);
    image.save(output_path).expect("Failed to save image");
}
