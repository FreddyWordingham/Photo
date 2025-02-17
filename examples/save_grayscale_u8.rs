use photo::ImageG;

const OUTPUT_DIR: &str = "output";
const IMAGE_NAME: &str = "grayscale-u8.png";

fn main() {
    let filepath = format!("{}/{}", OUTPUT_DIR, IMAGE_NAME);

    let mut image = ImageG::<u8>::empty(40, 20);

    for n in 0..20 {
        image.set_pixel([n, n], [255]);
    }

    println!("{}", image.data);

    // let dv = 255 / 20;
    // let mut v = 0;
    // for n in 20..40 {
    //     v += dv;
    //     for m in 0..20 {
    //         image.set_pixel([n, m], [v]);
    //     }
    // }

    println!("{}", image);
    image.save(filepath).expect("Failed to save image");
}
