use photo::ImageRGB;
use std::any::type_name;

type T = u8;

const INPUT_DIR: &str = "input";

fn main() {
    let image_name: &str = &format!("colour-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = ImageRGB::<T>::load(filepath).expect("Failed to load image");

    println!("{}", image);
}
