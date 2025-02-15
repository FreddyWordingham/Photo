use ndarray_images::ImageRGBA;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "colour_alpha-u8.png";

fn main() {
    let filepath = format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);

    let mut image = ImageRGBA::<u8>::empty(40, 20);

    for n in 0..20 {
        image.set_pixel([n, n], [255, 0, 0, 255]);
    }

    let dv = 255 / 20;
    let mut v = 0;
    for n in 20..40 {
        v += dv;
        for m in 0..20 {
            image.set_pixel([n, m], [0, v, 255 - v, 255]);
        }
    }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
