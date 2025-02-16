use palette::LinSrgb;
use photo::Image;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "colour-linsrgb.png";

fn main() {
    let filepath = format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);

    let mut image = Image::<LinSrgb>::empty(40, 20);

    for n in 0..20 {
        image.set_pixel([n, n], LinSrgb::new(1.0, 0.0, 0.0));
    }

    let dv = 1.0 / 20.0;
    let mut v: f32 = 0.0;
    for n in 20..40 {
        v += dv;
        for m in 0..20 {
            image.set_pixel(
                [n, m],
                LinSrgb::new(0.0, v.clamp(0.0, 1.0), 1.0 - v.clamp(0.0, 1.0)),
            );
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
