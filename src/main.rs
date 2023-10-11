use photo::Image;

fn main() {
    let mut image = Image::new(40, 60, [0.1, 0.1, 0.1, 1.0]);

    image.set_rectangle(00 + 5, 00 + 5, 8, 4, [1.0, 0.0, 0.0, 1.0]);
    image.set_rectangle(10 + 5, 00 + 5, 8, 4, [0.7, 0.0, 0.0, 1.0]);
    image.set_rectangle(10 + 5, 10 + 5, 8, 4, [1.0, 0.0, 0.0, 1.0]);
    image.set_rectangle(20 + 5, 10 + 5, 8, 4, [0.9, 0.8, 0.0, 1.0]);
    image.set_rectangle(20 + 5, 20 + 5, 8, 4, [0.0, 1.0, 0.0, 1.0]);
    image.set_rectangle(20 + 5, 30 + 5, 8, 4, [0.0, 1.0, 1.0, 1.0]);
    image.set_rectangle(10 + 5, 30 + 5, 8, 4, [0.0, 0.0, 1.0, 1.0]);
    image.set_rectangle(00 + 5, 20 + 5, 8, 4, [1.0, 0.0, 1.0, 1.0]);
    image.set_rectangle(00 + 5, 30 + 5, 8, 4, [0.4, 0.0, 0.6, 1.0]);

    println!("{} rows - {} cols", image.nrows(), image.ncols());
    // println!("{:?}", image.as_slice());

    image.print([1.0, 1.0, 1.0]);
    // image.print([1.0, 0.0, 0.0]);
    // image.print([0.0, 1.0, 0.0]);
    // image.print([0.0, 0.0, 1.0]);
    // image.print_channel(2);

    image.save("output/test.png");
}
