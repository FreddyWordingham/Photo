use ndarray::Array2;

use image::Image;

fn example_save_image() {
    let width = 20;
    let height = 10;
    let mut image = Array2::zeros((height, width));

    for i in 0..height {
        image[[i, i]] = 1.0;
    }

    image.save("image.png");
}

fn example_load_image() {
    let image: Array2<f32> = Array2::load("image.png");
    println!("Width: {}", image.width());
    println!("Height: {}", image.height());
    println!("{:?}", image);
}

fn main() {
    example_save_image();
    example_load_image();
}
