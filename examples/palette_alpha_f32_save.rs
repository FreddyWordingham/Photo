use palette::LinSrgba;
use photo::{ColourMap, Image};
use std::any::type_name;

type T = f32;
const T_MAX: T = 1.0;
type C = LinSrgba<T>;

const OUTPUT_DIR: &str = "output";

const COLOURS: [&str; 7] = [
    "#466be3", "#29bbec", "#30f199", "#a3fd3d", "#edd03a", "#fb8023", "#d23104",
];

fn main() {
    let image_name: &str = &format!("palette_alpha-{}.png", type_name::<T>());
    let filepath = format!("{}/{}", OUTPUT_DIR, image_name);

    let height = 20;
    let width = 40;
    let mut image = Image::<C>::empty([height, width]);

    let delta = T_MAX / width as T;
    for x in 0..width {
        let v = delta * x as T;
        for y in 0..height {
            let p = C::new(v, v, v, v);
            image.set_pixel([y, x], p);
        }
    }

    let cmap = ColourMap::<T, C>::new(&COLOURS);
    let grades = 7;
    for n in 0..height {
        for g in 0..grades {
            let p = cmap.sample(g as T / (grades - 1) as T);
            image.set_pixel([n, n + g], p);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
