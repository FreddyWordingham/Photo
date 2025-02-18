use palette::LinSrgba;
use photo::Image;
use std::any::type_name;

type T = f64;
type C = LinSrgba<T>;

const INPUT_DIR: &str = "input";

fn main() {
    let image_name: &str = &format!("palette_alpha-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = Image::<C>::load(filepath).expect("Failed to load image");

    println!("{}", image);
}
