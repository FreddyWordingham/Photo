use ndarray::{Array2, Array3};
use photo::{Channels, Image};

#[test]
fn test_new_image() {
    let data = Array3::<f32>::from_shape_fn((4, 6, 3), |(row, col, ch)| (row + col + ch) as f32);
    let image = Image::new(&data);

    assert_eq!(image.format(), Channels::RGB);
    assert_eq!(image.resolution(), (4, 6));
    assert_eq!(image.height(), 4);
    assert_eq!(image.width(), 6);
}

#[test]
#[should_panic(expected = "Image height must be positive")]
fn test_new_image_zero_height() {
    let data = Array3::<f32>::from_shape_fn((0, 6, 3), |(_, col, ch)| (col + ch) as f32);
    let _ = Image::new(&data);
}

#[test]
#[should_panic(expected = "Image width must be positive")]
fn test_new_image_zero_width() {
    let data = Array3::<f32>::from_shape_fn((4, 0, 3), |(row, _, ch)| (row + ch) as f32);
    let _ = Image::new(&data);
}

#[test]
#[should_panic(expected = "Number of channels must be between 1 and 4")]
fn test_new_image_invalid_channels() {
    let data = Array3::<f32>::from_shape_fn((4, 6, 5), |(row, col, ch)| (row + col + ch) as f32);
    let _ = Image::new(&data);
}

#[test]
fn test_empty_image() {
    let image = Image::<f32>::empty((10, 15), Channels::RGBA);

    assert_eq!(image.format(), Channels::RGBA);
    assert_eq!(image.resolution(), (10, 15));

    assert!(image.data.iter().all(|&v| v == 0.0));
}

#[test]
#[should_panic(expected = "Height must be positive")]
fn test_empty_image_zero_height() {
    let _ = Image::<f32>::empty((0, 15), Channels::RGBA);
}

#[test]
#[should_panic(expected = "Width must be positive")]
fn test_empty_image_zero_width() {
    let _ = Image::<f32>::empty((10, 0), Channels::RGBA);
}

#[test]
fn test_filled_greyscale_image() {
    let values = vec![0.5];
    let image = Image::<f32>::filled((5, 8), &values);

    assert_eq!(image.format(), Channels::Grey);
    assert_eq!(image.resolution(), (5, 8));

    assert!(image.data.iter().all(|&v| v == 0.5));
}

#[test]
fn test_filled_rgb_image() {
    let values = vec![0.1, 0.2, 0.3];
    let image = Image::<f32>::filled((3, 4), &values);

    assert_eq!(image.format(), Channels::RGB);
    assert_eq!(image.resolution(), (3, 4));

    for i in 0..3 {
        for j in 0..4 {
            assert_eq!(image[(i, j, 0)], 0.1);
            assert_eq!(image[(i, j, 1)], 0.2);
            assert_eq!(image[(i, j, 2)], 0.3);
        }
    }
}

#[test]
#[should_panic(expected = "Height must be positive")]
fn test_filled_image_zero_height() {
    let values = vec![0.1, 0.2, 0.3];
    let _ = Image::<f32>::filled((0, 4), &values);
}

#[test]
#[should_panic(expected = "Width must be positive")]
fn test_filled_image_zero_width() {
    let values = vec![0.1, 0.2, 0.3];
    let _ = Image::<f32>::filled((3, 0), &values);
}

#[test]
#[should_panic(expected = "Number of channels must be between 1 and 4")]
fn test_filled_image_invalid_channels() {
    let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
    let _ = Image::<f32>::filled((3, 4), &values);
}

#[test]
fn test_from_layers() {
    let r_layer = Array2::<f32>::from_elem((3, 3), 1.0);
    let g_layer = Array2::<f32>::from_elem((3, 3), 0.5);
    let b_layer = Array2::<f32>::from_elem((3, 3), 0.0);

    let image = Image::<f32>::from_layers(&[r_layer, g_layer, b_layer]);

    assert_eq!(image.format(), Channels::RGB);
    assert_eq!(image.resolution(), (3, 3));

    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(image[(i, j, 0)], 1.0);
            assert_eq!(image[(i, j, 1)], 0.5);
            assert_eq!(image[(i, j, 2)], 0.0);
        }
    }
}

#[test]
#[should_panic(expected = "Number of layers must be between 1 and 4")]
fn test_from_layers_invalid_channels() {
    let layers = vec![
        Array2::<f32>::from_elem((3, 3), 1.0),
        Array2::<f32>::from_elem((3, 3), 0.5),
        Array2::<f32>::from_elem((3, 3), 0.0),
        Array2::<f32>::from_elem((3, 3), 0.8),
        Array2::<f32>::from_elem((3, 3), 0.9),
    ];
    let _ = Image::<f32>::from_layers(&layers);
}

#[test]
#[should_panic(expected = "Number of layers must be between 1 and 4")]
fn test_from_layers_empty() {
    let layers: Vec<Array2<f32>> = Vec::new();
    let _ = Image::<f32>::from_layers(&layers);
}

#[test]
#[should_panic(expected = "Image height must be positive")]
fn test_from_layers_zero_height() {
    let layers = vec![
        Array2::<f32>::from_shape_fn((0, 3), |_| 1.0),
        Array2::<f32>::from_shape_fn((0, 3), |_| 0.5),
        Array2::<f32>::from_shape_fn((0, 3), |_| 0.0),
    ];
    let _ = Image::<f32>::from_layers(&layers);
}

#[test]
#[should_panic(expected = "Image width must be positive")]
fn test_from_layers_zero_width() {
    let layers = vec![
        Array2::<f32>::from_shape_fn((3, 0), |_| 1.0),
        Array2::<f32>::from_shape_fn((3, 0), |_| 0.5),
        Array2::<f32>::from_shape_fn((3, 0), |_| 0.0),
    ];
    let _ = Image::<f32>::from_layers(&layers);
}

#[test]
#[should_panic(expected = "All layers must have the same dimensions")]
fn test_from_layers_inconsistent_dimensions() {
    let layers = vec![
        Array2::<f32>::from_elem((3, 3), 1.0),
        Array2::<f32>::from_elem((3, 4), 0.5),
        Array2::<f32>::from_elem((3, 3), 0.0),
    ];
    let _ = Image::<f32>::from_layers(&layers);
}
