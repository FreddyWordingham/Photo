use photo::ImageRGB;
use std::any::type_name;

type T = f64;
const T_MAX: T = 1.0;

const OUTPUT_DIR: &str = "output";

const COLOUR_MAP: [[T; 3]; 7] = [
    [0.2745098, 0.4196078, 0.8901961], // #466be3
    [0.1607843, 0.7333333, 0.9254902], // #29bbec
    [0.1882353, 0.9450980, 0.6000000], // #30f199
    [0.6392157, 0.9921569, 0.2392157], // #a3fd3d
    [0.9294118, 0.8156863, 0.2274510], // #edd03a
    [0.9843137, 0.5019608, 0.1372549], // #fb8023
    [0.8235294, 0.1921569, 0.0156863], // #d23104
];

fn main() {
    let image_name: &str = &format!("colour-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", OUTPUT_DIR, image_name);

    let height = 20;
    let width = 40;
    let mut image = ImageRGB::<T>::empty([height, width]);

    let delta = T_MAX / width as T;
    for x in 0..width {
        let v = delta * x as T;
        for y in 0..height {
            image.set_pixel([y, x], [v, v, v]);
        }
    }

    let grades = 7;
    for n in 0..height {
        for g in 0..grades {
            image.set_pixel([n, n + g], COLOUR_MAP[g as usize]);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
