use nav::Direction;
use ndarray::Array3;
use photo::{Channels, Image};

#[test]
fn test_view_border() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    // Test North border
    let north_border = image.view_border(&Direction::North, 1);
    assert_eq!(north_border.dim(), (1, 8, 1));
    for col in 0..8 {
        assert_eq!(north_border[(0, col, 0)], image[(0, col, 0)]);
    }

    // Test East border
    let east_border = image.view_border(&Direction::East, 1);
    assert_eq!(east_border.dim(), (6, 1, 1));
    for row in 0..6 {
        assert_eq!(east_border[(row, 0, 0)], image[(row, 7, 0)]);
    }

    // Test South border
    let south_border = image.view_border(&Direction::South, 1);
    assert_eq!(south_border.dim(), (1, 8, 1));
    for col in 0..8 {
        assert_eq!(south_border[(0, col, 0)], image[(5, col, 0)]);
    }

    // Test West border
    let west_border = image.view_border(&Direction::West, 1);
    assert_eq!(west_border.dim(), (6, 1, 1));
    for row in 0..6 {
        assert_eq!(west_border[(row, 0, 0)], image[(row, 0, 0)]);
    }
}

#[test]
#[should_panic(expected = "Border size must be positive")]
fn test_view_border_zero_size() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border(&Direction::North, 0);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when viewing northern border")]
fn test_view_north_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border(&Direction::North, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when viewing eastern border")]
fn test_view_east_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border(&Direction::East, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when viewing southern border")]
fn test_view_south_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border(&Direction::South, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when viewing western border")]
fn test_view_west_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border(&Direction::West, 6);
}

#[test]
fn test_view_border_mut() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let mut image = Image::new(&data);

    // Test North border
    {
        let mut north_border = image.view_border_mut(&Direction::North, 1);
        assert_eq!(north_border.dim(), (1, 8, 1));

        for col in 0..8 {
            north_border[(0, col, 0)] += 1.0;
        }
        for col in 1..7 {
            assert_eq!(image[(0, col, 0)], (0 * 10 + col) as f32 + 1.0);
        }
    }
    // Test East border
    {
        let mut east_border = image.view_border_mut(&Direction::East, 1);
        assert_eq!(east_border.dim(), (6, 1, 1));

        for row in 0..6 {
            east_border[(row, 0, 0)] += 1.0;
        }
        for row in 1..5 {
            assert_eq!(image[(row, 7, 0)], (row * 10 + 7) as f32 + 1.0);
        }
    }
    // Test South border
    {
        let mut south_border = image.view_border_mut(&Direction::South, 1);
        assert_eq!(south_border.dim(), (1, 8, 1));

        for col in 0..8 {
            south_border[(0, col, 0)] += 1.0;
        }
        for col in 1..7 {
            assert_eq!(image[(5, col, 0)], (5 * 10 + col) as f32 + 1.0);
        }
    }
    // Test West border
    {
        let mut west_border = image.view_border_mut(&Direction::West, 1);
        assert_eq!(west_border.dim(), (6, 1, 1));

        for row in 0..6 {
            west_border[(row, 0, 0)] += 1.0;
        }
        for row in 1..5 {
            assert_eq!(image[(row, 0, 0)], (row * 10 + 0) as f32 + 1.0);
        }
    }
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when viewing northern border")]
fn test_view_north_border_mut_too_large() {
    let mut image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border_mut(&Direction::North, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when viewing eastern border")]
fn test_view_east_border_mut_too_large() {
    let mut image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border_mut(&Direction::East, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when viewing southern border")]
fn test_view_south_border_mut_too_large() {
    let mut image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border_mut(&Direction::South, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when viewing western border")]
fn test_view_west_border_mut_too_large() {
    let mut image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_border_mut(&Direction::West, 6);
}

#[test]
fn test_copy_border() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    // Test North border
    let north_border = image.copy_border(&Direction::North, 1);
    assert_eq!(north_border.format(), Channels::Grey);
    assert_eq!(north_border.resolution(), (1, 8));
    for col in 0..8 {
        assert_eq!(north_border[(0, col, 0)], image[(0, col, 0)]);
    }

    // Test East border
    let east_border = image.copy_border(&Direction::East, 1);
    assert_eq!(east_border.format(), Channels::Grey);
    assert_eq!(east_border.resolution(), (6, 1));
    for row in 0..6 {
        assert_eq!(east_border[(row, 0, 0)], image[(row, 7, 0)]);
    }

    // Test South border
    let south_border = image.copy_border(&Direction::South, 1);
    assert_eq!(south_border.format(), Channels::Grey);
    assert_eq!(south_border.resolution(), (1, 8));
    for col in 0..8 {
        assert_eq!(south_border[(0, col, 0)], image[(5, col, 0)]);
    }

    // Test West border
    let west_border = image.copy_border(&Direction::West, 1);
    assert_eq!(west_border.format(), Channels::Grey);
    assert_eq!(west_border.resolution(), (6, 1));
    for row in 0..6 {
        assert_eq!(west_border[(row, 0, 0)], image[(row, 0, 0)]);
    }
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when copying northern border")]
fn test_copy_north_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.copy_border(&Direction::North, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when copying eastern border")]
fn test_copy_east_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.copy_border(&Direction::East, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to height when copying southern border")]
fn test_copy_south_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.copy_border(&Direction::South, 6);
}

#[test]
#[should_panic(expected = "Border size must be less than or equal to width when copying western border")]
fn test_copy_west_border_too_large() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.copy_border(&Direction::West, 6);
}
