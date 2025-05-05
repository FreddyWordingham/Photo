use chromatic::HsvAlpha;
use ndarray::Array2;
use photo::Image;
use std::{fs::create_dir_all, path::Path};
use vista::{DisplayExt, DoubleJoined};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Path::new("input/colour_alpha-f32.png");

    // Load the image from the input path
    println!("Loading image from {:?}", input_path);
    let mut img: Array2<HsvAlpha<f32>> = Array2::load(input_path)?;

    // Print image to terminal
    let (height, width) = img.dim();
    println!("Image dimensions: {}x{}", width, height);
    println!("{}", img.display::<DoubleJoined>());

    // Modify the image
    img.mapv_inplace(|pixel| {
        HsvAlpha::new(
            pixel.hue() + 60.0, // Shift hue by 60 degrees
            pixel.saturation(),
            pixel.value(),
            pixel.alpha(),
        )
    });

    // Print the modified image to terminal
    println!("Modified image:");
    println!("{}", img.display::<DoubleJoined>());

    // Create the output directory if it doesn't exist
    create_dir_all("output")?;

    // Save the modified image to a new file
    let output_path = Path::new("output/image.png");
    img.save(output_path)?;
    println!("Saved modified image to {:?}", output_path);

    Ok(())
}
