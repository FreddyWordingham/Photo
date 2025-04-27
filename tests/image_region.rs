use ndarray::Array3;
use photo::{Channels, Image};

#[test]
fn test_view_region() {
    let data = Array3::<f32>::from_shape_fn((8, 8, 3), |(row, col, ch)| (row * 10 + col) as f32 + ch as f32 * 0.1);
    let image = Image::new(&data);

    let region_view = image.view_region((2, 3), (3, 4));
    assert_eq!(region_view.dim(), (3, 4, 3));
    for row in 0..3 {
        for col in 0..4 {
            for ch in 0..3 {
                assert_eq!(region_view[[row, col, ch]], image[(row + 2, col + 3, ch)]);
            }
        }
    }
}

#[test]
#[should_panic(expected = "Region height must be positive")]
fn test_view_region_zero_height() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);
    let _ = image.view_region((0, 0), (0, 2));
}

#[test]
#[should_panic(expected = "Region width must be positive")]
fn test_view_region_zero_width() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);
    let _ = image.view_region((0, 0), (2, 0));
}

#[test]
#[should_panic(expected = "Region exceeds image height")]
fn test_view_region_exceeds_height() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);
    let _ = image.view_region((1, 1), (3, 2));
}

#[test]
#[should_panic(expected = "Region exceeds image width")]
fn test_view_region_exceeds_width() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);
    let _ = image.view_region((1, 1), (2, 3));
}

#[test]
fn test_view_region_mut() {
    let data = Array3::<f32>::from_shape_fn((8, 8, 3), |(row, col, ch)| (row * 10 + col) as f32 + ch as f32 * 0.1);
    let mut image = Image::new(&data);

    let mut region_view_mut = image.view_region_mut((2, 3), (3, 4));
    assert_eq!(region_view_mut.dim(), (3, 4, 3));

    for row in 0..3 {
        for col in 0..4 {
            for ch in 0..3 {
                region_view_mut[[row, col, ch]] += 1.0;
            }
        }
    }

    for row in 0..3 {
        for col in 0..4 {
            for ch in 0..3 {
                if row < 2 || row > 5 || col < 3 || col > 7 {
                    assert_eq!(image[(row, col, ch)], (row * 10 + col) as f32 + ch as f32 * 0.1);
                } else {
                    assert_eq!(image[(row, col, ch)], (row * 10 + col) as f32 + ch as f32 * 0.1 + 1.0);
                }
            }
        }
    }
}

#[test]
fn test_view_copy_region() {
    let data = Array3::<f32>::from_shape_fn((8, 8, 3), |(row, col, ch)| (row * 10 + col) as f32 + ch as f32 * 0.1);
    let image = Image::new(&data);

    let region_copy = image.copy_region((2, 3), (3, 4));

    assert_eq!(region_copy.format(), Channels::RGB);
    assert_eq!(region_copy.resolution(), (3, 4));
    for row in 0..3 {
        for col in 0..4 {
            for ch in 0..3 {
                assert_eq!(region_copy[(row, col, ch)], image[(row + 2, col + 3, ch)]);
            }
        }
    }
}

#[test]
fn test_copy_region_wrapped() {
    let data = Array3::<u8>::from_shape_fn((4, 4, 1), |(row, col, _)| (row * 10 + col) as u8);
    let image = Image::new(&data);

    let wrapped_region = image.copy_region_wrapped((-1, -1), (3, 3));

    // Check dimensions
    assert_eq!(wrapped_region.resolution(), (3, 3));

    assert_eq!(wrapped_region[(0, 0, 0)], 33); // (-1, -1) wraps to (3, 3)
    assert_eq!(wrapped_region[(0, 1, 0)], 30); // (-1, 0) wraps to (3, 0)
    assert_eq!(wrapped_region[(0, 2, 0)], 31); // (-1, 1) wraps to (3, 1)

    assert_eq!(wrapped_region[(1, 0, 0)], 3); // (0, -1) wraps to (0, 3)
    assert_eq!(wrapped_region[(1, 1, 0)], 0); // (0, 0) is directly in the image
    assert_eq!(wrapped_region[(1, 2, 0)], 1); // (0, 1) is directly in the image

    assert_eq!(wrapped_region[(2, 0, 0)], 13); // (1, -1) wraps to (1, 3)
    assert_eq!(wrapped_region[(2, 1, 0)], 10); // (1, 0) is directly in the image
    assert_eq!(wrapped_region[(2, 2, 0)], 11); // (1, 1) is directly in the image
}
