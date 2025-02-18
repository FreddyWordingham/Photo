use palette::LinSrgb;
use photo::Image;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "colour-linsrgb.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = Image::<LinSrgb>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
