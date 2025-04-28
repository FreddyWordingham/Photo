use nav::Direction;
use ndarray::{Array2, Array3};
use photo::Image;
use std::{fs, path::Path};

/// Integration test for the image I/O functionality.
/// This test demonstrates a complete workflow:
/// 1. Create a test image
/// 2. Save the image to a PNG file
/// 3. Load the image back
/// 4. Perform multiple image manipulations
/// 5. Save the resulting image
/// 6. Verify the results
#[test]
fn test_integration_io() {
    // Create a test directory
    let test_dir = Path::new("integration_test_output_unique");
    if test_dir.exists() {
        // Clean up any existing files in the directory
        if let Ok(entries) = fs::read_dir(test_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let _ = fs::remove_file(path);
                    }
                }
            }
        }
        // Try to remove the directory, ignore if it fails
        let _ = fs::remove_dir(test_dir);
    }

    // Create the directory fresh
    fs::create_dir(test_dir).unwrap();

    // Create paths for our test files
    let original_path = test_dir.join("original.png");
    let grayscale_path = test_dir.join("grayscale.png");
    let red_channel_path = test_dir.join("red_channel.png");
    let green_channel_path = test_dir.join("green_channel.png");
    let blue_channel_path = test_dir.join("blue_channel.png");
    let region_path = test_dir.join("region.png");
    let stacked_path = test_dir.join("stacked.png");
    let final_result_path = test_dir.join("final_result.png");

    // Step 1: Create a test image (checkerboard pattern)
    let data = Array3::<u8>::from_shape_fn((200, 200, 3), |(row, col, ch)| {
        let checker = (row / 20 + col / 20) % 2 == 0;
        match (ch, checker) {
            (0, true) => 200u8,  // Red component in light squares
            (1, false) => 200u8, // Green component in dark squares
            (2, true) => 200u8,  // Blue component in light squares
            _ => 50u8,           // Lower values for other components
        }
    });
    let original_image = Image::new(&data);

    // Step 2: Save the original image
    original_image.save(&original_path).unwrap();
    println!("Original image saved to {:?}", original_path);

    // Step 3: Load the image back
    let loaded_image = Image::<u8>::load(&original_path).unwrap();

    // Verify the loaded image matches the original
    assert_eq!(loaded_image.format(), original_image.format());
    assert_eq!(loaded_image.resolution(), original_image.resolution());

    // Step 4: Perform image manipulations

    // 4.1: Extract individual channels and save them as grayscale images
    let red = loaded_image.get_channel(0);
    let green = loaded_image.get_channel(1);
    let blue = loaded_image.get_channel(2);

    // Create grayscale images from each channel
    let red_image = Image::new(&red.insert_axis(ndarray::Axis(2)).to_owned());
    let green_image = Image::new(&green.insert_axis(ndarray::Axis(2)).to_owned());
    let blue_image = Image::new(&blue.insert_axis(ndarray::Axis(2)).to_owned());

    // Save the channel images
    red_image.save(&red_channel_path).unwrap();
    green_image.save(&green_channel_path).unwrap();
    blue_image.save(&blue_channel_path).unwrap();
    println!("Individual channels saved as grayscale images");

    // 4.2: Convert to grayscale (average of channels)
    let grayscale_data = Array2::from_shape_fn(loaded_image.resolution(), |(row, col)| {
        // Simple average for grayscale conversion
        ((red[(row, col)] as u16 + green[(row, col)] as u16 + blue[(row, col)] as u16) / 3) as u8
    });

    let grayscale_image = Image::new(&grayscale_data.insert_axis(ndarray::Axis(2)).to_owned());

    grayscale_image.save(&grayscale_path).unwrap();
    println!("Grayscale image saved to {:?}", grayscale_path);

    // 4.3: Extract a region
    let region = loaded_image.copy_region((50, 50), (100, 100));
    region.save(&region_path).unwrap();
    println!("Region extracted and saved to {:?}", region_path);

    // 4.4: Create image tiles and stack them
    let tiles = region.copy_tiles((50, 50), (0, 0));
    let stacked = Image::stack(&tiles);
    stacked.save(&stacked_path).unwrap();
    println!("Tiled and stacked image saved to {:?}", stacked_path);

    // 4.5: Create a complex composition
    // Create borders
    let north_border = stacked.copy_border(&Direction::North, 10);
    let south_border = stacked.copy_border(&Direction::South, 10);

    // Stack them together
    let combined = Image::vstack(&[north_border, stacked, south_border]);

    // Apply a slide transform (wrapping pixels)
    let final_result = combined.copy_slide((20, -30));
    final_result.save(&final_result_path).unwrap();
    println!("Final result saved to {:?}", final_result_path);

    // Step 5: Verify the final result
    let verified_result = Image::<u8>::load(&final_result_path).unwrap();
    assert_eq!(verified_result.resolution(), final_result.resolution());
    assert_eq!(verified_result.format(), final_result.format());

    // Verify a few pixels to ensure the content matches
    for row in 0..5 {
        for col in 0..5 {
            for ch in 0..verified_result.format().num_channels() {
                assert_eq!(verified_result[(row, col, ch)], final_result[(row, col, ch)]);
            }
        }
    }

    println!("Integration test completed successfully!");

    // Clean up test files - check if files exist before trying to remove them
    for path in &[
        &original_path,
        &grayscale_path,
        &red_channel_path,
        &green_channel_path,
        &blue_channel_path,
        &region_path,
        &stacked_path,
        &final_result_path,
    ] {
        if path.exists() {
            let _ = fs::remove_file(path);
        }
    }

    // Remove directory if it exists
    if test_dir.exists() {
        let _ = fs::remove_dir(test_dir);
    }
}
