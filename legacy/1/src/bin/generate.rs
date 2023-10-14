use photo::Image;

use rand;

fn main() {
    println!("Hello, world!");
    let ncols = 400;
    let nrows = 300;
    let mut image = Image::new(nrows, ncols, [1.0, 0.0, 0.0, 1.0]);

    for _ in 0..100 {
        let col = rand::random::<usize>() % image.ncols();
        let row = rand::random::<usize>() % image.nrows();
        let r = rand::random::<usize>() % 25;
        image.draw_circle(row, col, r, [0.0, 1.0, 0.0, 1.0]);
    }

    println!("=> {} {}", image.nrows(), image.ncols());
    let high_dim = std::cmp::min(image.nrows(), image.ncols());
    for n in 0..(high_dim - 19) {
        println!("=> {} {}", n, high_dim);
        image.draw_pixel(n, n, [0.0, 0.0, 1.0, 1.0]);
    }

    // let image_data = image.as_1d_f32();
    // image.from_1d_f32(&image_data);

    image.save("output/image.png");

    let _image = Image::load("output/image.png");
}
