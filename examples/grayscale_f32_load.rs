use photo::ImageG;
use std::any::type_name;

type T = f32;

const INPUT_DIR: &str = "input";

fn main() {
    let image_name: &str = &format!("grayscale-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = ImageG::<T>::load(filepath).expect("Failed to load image");

    println!("{}", image);
}
