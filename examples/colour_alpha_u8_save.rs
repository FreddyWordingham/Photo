use photo::ImageRGBA;
use std::any::type_name;

type T = u8;
const T_MAX: T = 255;

const OUTPUT_DIR: &str = "output";

const COLOUR_MAP: [[T; 3]; 7] = [
    [70, 107, 227], // #466be3
    [41, 187, 236], // #29bbec
    [48, 241, 153], // #30f199
    [163, 253, 61], // #a3fd3d
    [237, 208, 58], // #edd03a
    [251, 128, 35], // #fb8023
    [210, 49, 4],   // #d23104
];

fn main() {
    let image_name: &str = &format!("colour_alpha-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", OUTPUT_DIR, image_name);

    let height = 20;
    let width = 40;
    let mut image = ImageRGBA::<T>::empty([height, width]);

    let delta = T_MAX / width as T;
    for x in 0..width {
        let v = delta * x as T;
        for y in 0..height {
            image.set_pixel([y, x], [v, v, v, v]);
        }
    }

    let grades = 7;
    for n in 0..height {
        for g in 0..grades {
            let p = [
                COLOUR_MAP[g as usize][0],
                COLOUR_MAP[g as usize][1],
                COLOUR_MAP[g as usize][2],
                T_MAX,
            ];
            image.set_pixel([n, n + g], p);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
