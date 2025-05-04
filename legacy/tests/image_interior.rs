use ndarray::Array3;
use photo::Image;

#[test]
fn test_view_interior() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    let interior = image.view_interior(1);
    assert_eq!(interior.dim(), (4, 6, 1));
    for row in 0..4 {
        for col in 0..6 {
            assert_eq!(interior[(row, col, 0)], image[(row + 1, col + 1, 0)]);
        }
    }
}

#[test]
#[should_panic(expected = "Border size must be positive")]
fn test_view_interior_zero_border() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_interior(0);
}

#[test]
#[should_panic(expected = "Border size must be less than half the height")]
fn test_view_interior_border_too_large_height() {
    let image = Image::<f32>::filled((5, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_interior(3);
}

#[test]
#[should_panic(expected = "Border size must be less than half the width")]
fn test_view_interior_border_too_large_width() {
    let image = Image::<f32>::filled((7, 5), &[0.1, 0.2, 0.3]);
    let _ = image.view_interior(3);
}

#[test]
fn test_view_interior_mut() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let mut image = Image::new(&data);

    let mut interior = image.view_interior_mut(1);
    interior.fill(99.0);
    for row in 0..6 {
        for col in 0..8 {
            if row >= 1 && row < 5 && col >= 1 && col < 7 {
                assert_eq!(image[(row, col, 0)], 99.0);
            } else {
                assert_ne!(image[(row, col, 0)], 99.0);
            }
        }
    }
}

#[test]
fn test_copy_interior() {
    let data = Array3::<f32>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as f32);
    let image = Image::new(&data);

    let interior = image.copy_interior(1);
    assert_eq!(interior.resolution(), (4, 6));
    assert_eq!(interior.format(), image.format());

    for row in 0..4 {
        for col in 0..6 {
            assert_eq!(interior[(row, col, 0)], image[(row + 1, col + 1, 0)]);
        }
    }
}
