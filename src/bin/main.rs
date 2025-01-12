use ndarray::{Array2, Array3};

use image::Image;

#[allow(dead_code)]
fn example_save_grayscale_image() {
    let width = 20;
    let height = 10;
    let mut image = Array2::zeros((height, width));

    for i in 0..height {
        image[[i, i]] = 1.0;
    }

    image.save("image.png");
}

#[allow(dead_code)]
fn example_load_grayscale_image() {
    let image: Array2<f32> = Array2::load("image.png");
    println!("Width: {}", image.width());
    println!("Height: {}", image.height());
    println!("{:?}", image);
}

#[allow(dead_code)]
fn example_save_color_image() {
    let width = 20;
    let height = 10;
    let components = 4;
    let mut image = Array3::zeros((height, width, components));

    for i in 0..height {
        image[[i, i, 0]] = 1.0;
        image[[i, i, 1]] = 0.0;
        image[[i, i, 2]] = 0.0;
        image[[i, i, 3]] = 1.0;
    }

    image.save("image.png");
}

#[allow(dead_code)]
fn example_load_color_image() {
    let image: Array3<f32> = Array3::load("image.png");
    println!("Width: {}", image.width());
    println!("Height: {}", image.height());
    println!("{:?}", image);
}

fn main() {
    // example_save_grayscale_image();
    // example_load_grayscale_image();
    example_save_color_image();
    example_load_color_image();
}
