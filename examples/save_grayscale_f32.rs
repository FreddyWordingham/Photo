use photo::ImageG;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "grayscale-f32.png";

fn main() {
    let filepath = format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);

    let mut image = ImageG::<f32>::empty(40, 20);

    for n in 0..20 {
        image.set_pixel([n, n], [1.0]);
    }

    let dv = 1.0 / 20.0;
    let mut v: f32 = 0.0;
    for n in 20..40 {
        v += dv;
        for m in 0..20 {
            image.set_pixel([n, m], [v.clamp(0.0, 1.0)]);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
