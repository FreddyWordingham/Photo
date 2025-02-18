use photo::ImageRGBA;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "colour_alpha-f32.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageRGBA::<u8>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
