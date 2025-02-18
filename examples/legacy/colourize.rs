use palette::LinSrgba;
use photo::{ColourMap, ImageG};

const INPUT_DIR: &str = "input";
const IMAGE_NAME: &str = "grayscale-u8.png";

fn main() {
    let hex_colours = vec!["#FF000000", "#00FF00", "#0000FF"];
    let cmap: ColourMap<f32, LinSrgba> = ColourMap::new(&hex_colours);

    let filepath = format!("{}/{}", INPUT_DIR, IMAGE_NAME);
    let image = ImageG::<f32>::load(filepath).expect("Failed to load image");
    let coloured_image = image.colourize(&cmap);

    println!("{}", coloured_image);
}
