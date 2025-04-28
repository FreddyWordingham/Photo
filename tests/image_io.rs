use ndarray::Array3;
use photo::{Channels, Image};
use std::{fs, path::Path};

#[test]
fn test_save_load_rgb_image() {
    let test_dir = Path::new("test_output_rgb");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }

    let test_path = test_dir.join("test_rgb.png");

    let data = Array3::<u8>::from_shape_fn((100, 100, 3), |(row, col, ch)| match ch {
        0 => (row * 255 / 100) as u8,         // Red varies with row
        1 => (col * 255 / 100) as u8,         // Green varies with column
        2 => ((row + col) * 128 / 200) as u8, // Blue varies with both
        _ => unreachable!(),
    });
    let image = Image::new(&data);

    image.save(&test_path).unwrap();

    let loaded_image = Image::<u8>::load(&test_path).unwrap();
    assert_eq!(loaded_image.format(), Channels::RGB);
    assert_eq!(loaded_image.resolution(), (100, 100));
    for row in 0..100 {
        for col in 0..100 {
            assert_eq!(loaded_image[(row, col, 0)], (row * 255 / 100) as u8);
            assert_eq!(loaded_image[(row, col, 1)], (col * 255 / 100) as u8);
            assert_eq!(loaded_image[(row, col, 2)], ((row + col) * 128 / 200) as u8);
        }
    }

    if test_path.exists() {
        fs::remove_file(test_path).unwrap();
    }
    if test_dir.exists() {
        fs::remove_dir(test_dir).unwrap();
    }
}

#[test]
fn test_save_load_grayscale_image() {
    let test_dir = Path::new("test_output_gray");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }

    let test_path = test_dir.join("test_gray.png");

    let data = Array3::<u8>::from_shape_fn((50, 50, 1), |(row, col, _)| ((row + col) * 255 / 100) as u8);
    let image = Image::new(&data);

    image.save(&test_path).unwrap();

    let loaded_image = Image::<u8>::load(&test_path).unwrap();
    assert_eq!(loaded_image.format(), Channels::Grey);
    assert_eq!(loaded_image.resolution(), (50, 50));
    for row in 0..50 {
        for col in 0..50 {
            assert_eq!(loaded_image[(row, col, 0)], ((row + col) * 255 / 100) as u8);
        }
    }

    if test_path.exists() {
        fs::remove_file(test_path).unwrap();
    }
    if test_dir.exists() {
        fs::remove_dir(test_dir).unwrap();
    }
}

#[test]
fn test_extract_channel_as_grayscale() {
    let test_dir = Path::new("test_output_channel");
    if !test_dir.exists() {
        fs::create_dir(test_dir).unwrap();
    }

    let test_path = test_dir.join("test_channel.png");

    let data = Array3::<u8>::from_shape_fn((80, 80, 3), |(row, col, ch)| match ch {
        0 => (row * 255 / 80) as u8, // Red varies with row
        1 => (col * 255 / 80) as u8, // Green varies with column
        2 => 128u8,                  // Blue is constant
        _ => unreachable!(),
    });
    let image = Image::new(&data);

    let green_channel = image.get_channel(1);
    let grayscale_image = Image::new(&green_channel.insert_axis(ndarray::Axis(2)).to_owned());

    grayscale_image.save(&test_path).unwrap();

    let loaded_image = Image::<u8>::load(&test_path).unwrap();
    assert_eq!(loaded_image.format(), Channels::Grey);
    assert_eq!(loaded_image.resolution(), (80, 80));
    for row in 0..80 {
        for col in 0..80 {
            assert_eq!(loaded_image[(row, col, 0)], (col * 255 / 80) as u8);
        }
    }

    if test_path.exists() {
        fs::remove_file(test_path).unwrap();
    }
    if test_dir.exists() {
        fs::remove_dir(test_dir).unwrap();
    }
}
