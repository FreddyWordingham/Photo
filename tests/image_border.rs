use nav::Direction;
use ndarray::Array3;
use photo::{Channels, Image};

#[test]
fn test_view_border() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    let north_border = image.view_border(&Direction::North, 1);

    assert_eq!(north_border.dim(), (1, 8, 1));
    for col in 0..8 {
        assert_eq!(north_border[(0, col, 0)], image[(0, col, 0)]);
    }

    let south_border = image.copy_border(&Direction::South, 1);

    assert_eq!(south_border.format(), Channels::Grey);
    assert_eq!(south_border.resolution(), (1, 8));
    for col in 0..8 {
        assert_eq!(south_border[(0, col, 0)], image[(5, col, 0)]);
    }
}

#[test]
fn test_view_border_mut() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let mut image = Image::new(&data);

    let mut east_border = image.view_border_mut(&Direction::East, 1);
    assert_eq!(east_border.dim(), (6, 1, 1));

    for row in 0..6 {
        east_border[(row, 0, 0)] += 1.0;
    }

    for row in 0..6 {
        for col in 5..8 {
            if col == 7 {
                assert_eq!(image[(row, col, 0)], (row * 10 + col) as f32 + 1.0);
            } else {
                assert_eq!(image[(row, col, 0)], (row * 10 + col) as f32);
            }
        }
    }

    let mut west_border = image.view_border_mut(&Direction::West, 1);
    assert_eq!(west_border.dim(), (6, 1, 1));

    for row in 0..6 {
        west_border[(row, 0, 0)] += 1.0;
    }

    for row in 0..6 {
        for col in 0..5 {
            if col == 0 {
                assert_eq!(image[(row, col, 0)], (row * 10 + col) as f32 + 1.0);
            } else {
                assert_eq!(image[(row, col, 0)], (row * 10 + col) as f32);
            }
        }
    }
}

#[test]
fn test_copy_border() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    let south_border = image.copy_border(&Direction::South, 1);

    assert_eq!(south_border.format(), Channels::Grey);
    assert_eq!(south_border.resolution(), (1, 8));
    for col in 0..8 {
        assert_eq!(south_border[(0, col, 0)], image[(5, col, 0)]);
    }
}
