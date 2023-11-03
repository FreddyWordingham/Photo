use photo;

fn main() {
    let width = 512;
    let height = 256;

    let settings = photo::Settings::new([width, height]);

    photo::run(&settings);
}
