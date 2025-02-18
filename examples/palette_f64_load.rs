use palette::LinSrgb;
use photo::Image;
use std::any::type_name;

type T = f64;
type C = LinSrgb<T>;

const INPUT_DIR: &str = "input";

fn main() {
    let image_name: &str = &format!("palette-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = Image::<C>::load(filepath).expect("Failed to load image");

    println!("{}", image);
}
