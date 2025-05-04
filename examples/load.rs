use chromatic::HsvAlpha;
use ndarray::{Array2, Zip};
use photo::Image;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to input and output files
    let input_path = Path::new("input/input.png");
    let output_path = Path::new("input/output.png");

    // Load the image from the input path
    println!("Loading image from {:?}", input_path);
    let img: Array2<HsvAlpha<f32>> = Array2::load(input_path)?;

    // Print image dimensions
    let (height, width) = img.dim();
    println!("Image dimensions: {}x{}", width, height);

    // Create a new image for the result
    let mut result = img.clone();

    // Apply some operation to the image (e.g., brightness adjustment)
    // This example increases brightness by 20%
    println!("Adjusting brightness...");
    Zip::from(&mut result).for_each(|pixel| {
        // Extract RGB components
        let h = pixel.hue();
        let s = pixel.saturation();
        let v = pixel.value();
        let a = pixel.alpha();
        // Ensure the alpha channel is preserved

        // Increase brightness by 20% with clamping to [0, 1]
        let new_h = h - 20.0;
        let new_s = s;
        let new_v = v; // Increase brightness by 20%
        let new_a = a; // Preserve alpha channel

        // Update the pixel
        *pixel = HsvAlpha::new(new_h, new_s, new_v, new_a);
    });

    // Save the result
    println!("Saving result to {:?}", output_path);
    Array2::save(&result, output_path)?;

    println!("Done!");
    Ok(())
}
