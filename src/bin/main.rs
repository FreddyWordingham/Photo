use photo::Image;

fn main() {
    println!("Hello, world!");
    let image = Image::new(400, 300, [1.0, 0.0, 0.0, 1.0]);
    image.save("a/b/g/s/zoiks/zoiks/image.png");
}
