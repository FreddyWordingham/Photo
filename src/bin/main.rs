use photo::Image;

use rand;

fn main() {
    println!("Hello, world!");
    let mut image = Image::new(400, 300, [1.0, 0.0, 0.0, 1.0]);

    for _ in 0..100 {
        let x = rand::random::<usize>() % image.width();
        let y = rand::random::<usize>() % image.height();
        let r = rand::random::<usize>() % 25;
        image.draw_circle(y, x, r, [0.0, 1.0, 0.0, 1.0]);
    }

    image.save("output/image.png");
}
