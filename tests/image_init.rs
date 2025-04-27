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
fn test_empty_image() {
    let image = Image::<f32>::empty((10, 15), Channels::RGBA);

    assert_eq!(image.format(), Channels::RGBA);
    assert_eq!(image.resolution(), (10, 15));

    assert!(image.data.iter().all(|&v| v == 0.0));
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
fn test_filled_image() {
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
