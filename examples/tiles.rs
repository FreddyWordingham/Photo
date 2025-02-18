use photo::ImageRGBA;

const INPUT_DIR: &str = "input";
const TILE_SIZE: [usize; 2] = [5, 5];

fn main() {
    let image_name = "colour_alpha-u8.png";
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = ImageRGBA::<u8>::load(filepath).expect("Failed to load image");
    println!("{}", image);
    println!("Height {}", image.height());
    println!("Width {}", image.width());

    let tiles = image.tiles(TILE_SIZE);
    for ((y, x), tile) in tiles.indexed_iter() {
        println!("Tile [{}, {}]:", y, x);
        println!("{}", tile);
    }
}
