use photo;
use rand::Rng;

fn main() {
    let nrows = 600;
    let ncols = 800;
    let mut image = photo::Image::new(nrows, ncols, [0.1, 0.1, 0.1, 1.0]);
    println!("{} rows - {} cols", image.nrows(), image.ncols());

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let row = rng.gen_range(0..image.nrows());
        let col = rng.gen_range(0..image.ncols());
        let radius = rng.gen_range(1..10);
        let rgba = [1.0, 0.0, 1.0, 1.0];
        image.set_circle(row, col, radius, rgba)
    }

    image.save("output/test.png");
}
