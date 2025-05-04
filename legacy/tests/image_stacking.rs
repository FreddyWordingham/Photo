use ndarray::{Array2, arr2};
use photo::{Channels, Image};

#[test]
fn test_vertical_stacking() {
    let top = Image::<u8>::filled((2, 3), &[10, 20, 30]);
    let bottom = Image::<u8>::filled((2, 3), &[40, 50, 60]);

    let stacked = Image::vstack(&[top, bottom]);

    assert_eq!(stacked.format(), Channels::RGB);
    assert_eq!(stacked.resolution(), (4, 3));

    for row in 0..2 {
        for col in 0..3 {
            assert_eq!(stacked[(row, col, 0)], 10);
            assert_eq!(stacked[(row, col, 1)], 20);
            assert_eq!(stacked[(row, col, 2)], 30);
        }
    }
    for row in 2..4 {
        for col in 0..3 {
            assert_eq!(stacked[(row, col, 0)], 40);
            assert_eq!(stacked[(row, col, 1)], 50);
            assert_eq!(stacked[(row, col, 2)], 60);
        }
    }
}

#[test]
#[should_panic(expected = "At least one image is required")]
fn test_vertical_stack_empty() {
    let images: Vec<Image<f32>> = Vec::new();
    let _ = Image::vstack(&images);
}

#[test]
#[should_panic(expected = "All images must have the same width")]
fn test_vertical_stack_inconsistent_width() {
    let img1 = Image::<f32>::filled((2, 3), &[0.1, 0.2, 0.3]);
    let img2 = Image::<f32>::filled((2, 4), &[0.1, 0.2, 0.3]);
    let _ = Image::vstack(&[img1, img2]);
}

#[test]
#[should_panic(expected = "All images must have the same format")]
fn test_vertical_stack_inconsistent_format() {
    let img1 = Image::<f32>::filled((2, 3), &[0.1, 0.2, 0.3]);
    let img2 = Image::<f32>::filled((2, 3), &[0.1, 0.2]);
    let _ = Image::vstack(&[img1, img2]);
}

#[test]
fn test_horizontal_stacking() {
    let left = Image::<u8>::filled((3, 2), &[10, 20, 30]);
    let right = Image::<u8>::filled((3, 2), &[40, 50, 60]);

    let stacked = Image::hstack(&[left, right]);

    assert_eq!(stacked.format(), Channels::RGB);
    assert_eq!(stacked.resolution(), (3, 4));

    for i in 0..3 {
        for j in 0..2 {
            assert_eq!(stacked[(i, j, 0)], 10);
            assert_eq!(stacked[(i, j, 1)], 20);
            assert_eq!(stacked[(i, j, 2)], 30);
        }
    }
    for i in 0..3 {
        for j in 2..4 {
            assert_eq!(stacked[(i, j, 0)], 40);
            assert_eq!(stacked[(i, j, 1)], 50);
            assert_eq!(stacked[(i, j, 2)], 60);
        }
    }
}

#[test]
#[should_panic(expected = "At least one image is required")]
fn test_horizontal_stack_empty() {
    let images: Vec<Image<f32>> = Vec::new();
    let _ = Image::hstack(&images);
}

#[test]
#[should_panic(expected = "All images must have the same height")]
fn test_horizontal_stack_inconsistent_height() {
    let img1 = Image::<f32>::filled((2, 3), &[0.1, 0.2, 0.3]);
    let img2 = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);
    let _ = Image::hstack(&[img1, img2]);
}

#[test]
#[should_panic(expected = "All images must have the same format")]
fn test_horizontal_stack_inconsistent_format() {
    let img1 = Image::<f32>::filled((2, 3), &[0.1, 0.2, 0.3]);
    let img2 = Image::<f32>::filled((2, 3), &[0.1]);
    let _ = Image::hstack(&[img1, img2]);
}

#[test]
fn test_image_stack_from_tiles() {
    let tile1 = Image::<u8>::filled((2, 2), &[10, 20, 30]);
    let tile2 = Image::<u8>::filled((2, 2), &[40, 50, 60]);
    let tile3 = Image::<u8>::filled((2, 2), &[70, 80, 90]);
    let tile4 = Image::<u8>::filled((2, 2), &[100, 110, 120]);

    let tiles = arr2(&[[tile1, tile2], [tile3, tile4]]);

    let image = Image::stack(&tiles);

    assert_eq!(image.format(), Channels::RGB);
    assert_eq!(image.resolution(), (4, 4));

    // Top-left
    for i in 0..2 {
        for j in 0..2 {
            assert_eq!(image[(i, j, 0)], 10);
            assert_eq!(image[(i, j, 1)], 20);
            assert_eq!(image[(i, j, 2)], 30);
        }
    }
    // Top-right
    for i in 0..2 {
        for j in 2..4 {
            assert_eq!(image[(i, j, 0)], 40);
            assert_eq!(image[(i, j, 1)], 50);
            assert_eq!(image[(i, j, 2)], 60);
        }
    }
    // Bottom-left
    for i in 2..4 {
        for j in 0..2 {
            assert_eq!(image[(i, j, 0)], 70);
            assert_eq!(image[(i, j, 1)], 80);
            assert_eq!(image[(i, j, 2)], 90);
        }
    }
    // Bottom-right
    for i in 2..4 {
        for j in 2..4 {
            assert_eq!(image[(i, j, 0)], 100);
            assert_eq!(image[(i, j, 1)], 110);
            assert_eq!(image[(i, j, 2)], 120);
        }
    }
}

#[test]
#[should_panic(expected = "Tile grid height must be positive")]
fn test_stack_tiles_zero_grid_height() {
    let tiles = Array2::<Image<f32>>::from_shape_fn((0, 2), |_| Image::<f32>::filled((2, 2), &[0.1, 0.2, 0.3]));
    let _ = Image::stack(&tiles);
}

#[test]
#[should_panic(expected = "Tile grid width must be positive")]
fn test_stack_tiles_zero_grid_width() {
    let tiles = Array2::<Image<f32>>::from_shape_fn((2, 0), |_| Image::<f32>::filled((2, 2), &[0.1, 0.2, 0.3]));
    let _ = Image::stack(&tiles);
}

#[test]
#[should_panic(expected = "All tiles must have the same resolution")]
fn test_stack_tiles_inconsistent_dimensions() {
    let tile1 = Image::<f32>::filled((2, 2), &[0.1, 0.2, 0.3]);
    let tile2 = Image::<f32>::filled((2, 3), &[0.1, 0.2, 0.3]);
    let tiles = Array2::from_shape_vec((1, 2), vec![tile1, tile2]).unwrap();
    let _ = Image::stack(&tiles);
}

#[test]
#[should_panic(expected = "All tiles must have the same format")]
fn test_stack_tiles_inconsistent_format() {
    let tile1 = Image::<f32>::filled((2, 2), &[0.1, 0.2, 0.3]);
    let tile2 = Image::<f32>::filled((2, 2), &[0.1, 0.2]);
    let tiles = Array2::from_shape_vec((1, 2), vec![tile1, tile2]).unwrap();
    let _ = Image::stack(&tiles);
}
