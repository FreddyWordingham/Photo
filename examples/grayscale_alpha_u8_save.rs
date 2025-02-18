use photo::ImageGA;
use std::any::type_name;

type T = u8;
const T_MAX: T = 255;

const OUTPUT_DIR: &str = "output";

fn main() {
    let image_name: &str = &format!("grayscale_alpha-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", OUTPUT_DIR, image_name);

    let height = 20;
    let width = 40;
    let mut image = ImageGA::<T>::empty([height, width]);

    let delta = T_MAX / width as T;
    for x in 0..width {
        let v = delta * x as T;
        for y in 0..height {
            image.set_pixel([y, x], [v, v]);
        }
    }

    let grades = 7;
    for n in 0..height {
        for g in 0..grades {
            let v = (T_MAX as f32 * g as f32 / grades as f32) as T;
            image.set_pixel([n, n + g], [v, T_MAX]);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
