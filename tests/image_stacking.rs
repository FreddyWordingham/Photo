use ndarray::arr2;
use photo::{Channels, Image};

#[test]
fn test_vertical_stacking() {
    let top = Image::<u8>::filled((2, 3), &[10, 20, 30]);
    let bottom = Image::<u8>::filled((2, 3), &[40, 50, 60]);

    let stacked = Image::vstack(&[top, bottom]);

    assert_eq!(stacked.format(), Channels::RGB);
    assert_eq!(stacked.resolution(), (4, 3));

    for i in 0..2 {
        for j in 0..3 {
            assert_eq!(stacked[(i, j, 0)], 10);
            assert_eq!(stacked[(i, j, 1)], 20);
            assert_eq!(stacked[(i, j, 2)], 30);
        }
    }
    for i in 2..4 {
        for j in 0..3 {
            assert_eq!(stacked[(i, j, 0)], 40);
            assert_eq!(stacked[(i, j, 1)], 50);
            assert_eq!(stacked[(i, j, 2)], 60);
        }
    }
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
