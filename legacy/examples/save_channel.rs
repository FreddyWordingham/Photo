use ndarray::Axis;
use photo::Image;

fn main() {
    // Load an image from a PNG file
    let image = Image::<u8>::load("image.png").unwrap();

    // Save an image to a PNG file
    image.save("output.png").unwrap();

    // Extract and save just one channel as a grayscale image
    let red_channel = Image::new(&image.get_channel(0).insert_axis(Axis(2)));
    red_channel.save("red_channel.png").unwrap();
}
