use photo::ImageRGB;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "colour-f32.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageRGB::<f32>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
