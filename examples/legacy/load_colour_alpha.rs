use palette::LinSrgba;
use photo::Image;

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "colour_alpha-linsrgba.png";

fn main() {
    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = Image::<LinSrgba>::load(filepath).expect("Failed to load image");
    println!("{}", image);
}
