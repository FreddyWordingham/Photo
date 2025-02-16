use ndarray_images::ImageGA;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "grayscale_alpha-f64.png";

fn main() {
    let filepath = format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);

    let mut image = ImageGA::<f64>::empty(40, 20);

    for n in 0..20 {
        image.set_pixel([n, n], [1.0, 1.0]);
    }

    let dv = 1.0 / 20.0;
    let mut v: f64 = 0.0;
    for n in 20..40 {
        v += dv;
        for m in 0..20 {
            image.set_pixel([n, m], [v.clamp(0.0, 1.0), v.clamp(0.0, 1.0)]);
        }
    }

    println!("{}", image);

    image.save(filepath).expect("Failed to save image");
}
