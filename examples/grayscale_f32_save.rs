use photo::ImageG;
use std::any::type_name;

type T = f32;
const T_MAX: T = 1.0;

const OUTPUT_DIR: &str = "output";

fn main() {
    let image_name: &str = &format!("grayscale-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", OUTPUT_DIR, image_name);

    let height = 20;
    let width = 40;
    let mut image = ImageG::<T>::empty([height, width]);

    let delta = T_MAX / width as T;
    for x in 0..width {
        let v = delta * x as T;
        for y in 0..height {
            image.set_pixel([y, x], [v]);
        }
    }

    let grades = 7;
    for n in 0..height {
        for g in 0..grades {
            image.set_pixel(
                [n, n + g],
                [((T_MAX as f32 * g as f32 / grades as f32) as T)],
            );
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
