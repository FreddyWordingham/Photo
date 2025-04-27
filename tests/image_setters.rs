use ndarray::{Array2, arr1};
use photo::Image;

#[test]
fn test_set_channel() {
    let mut image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    let new_green = Array2::<f32>::from_elem((3, 3), 0.5);
    image.set_channel(1, &new_green);

    let updated_green = image.get_channel(1);
    assert!(updated_green.iter().all(|&v| v == 0.5));

    assert!(image.get_channel(0).iter().all(|&v| v == 0.1));
    assert!(image.get_channel(2).iter().all(|&v| v == 0.3));
}

#[test]
fn test_set_pixel() {
    let mut image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    let new_pixel = arr1(&[0.9, 0.8, 0.7]);
    image.set_pixel((1, 1), &new_pixel);

    let updated_pixel = image.get_pixel((1, 1));
    assert_eq!(updated_pixel[0], 0.9);
    assert_eq!(updated_pixel[1], 0.8);
    assert_eq!(updated_pixel[2], 0.7);

    let other_pixel = image.get_pixel((0, 0));
    assert_eq!(other_pixel[0], 0.1);
    assert_eq!(other_pixel[1], 0.2);
    assert_eq!(other_pixel[2], 0.3);
}

#[test]
fn test_set_component() {
    let mut image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    image[(1, 1, 0)] = 0.9;
    image[(1, 1, 1)] = 0.8;
    image[(1, 1, 2)] = 0.7;

    assert_eq!(image[(1, 1, 0)], 0.9);
    assert_eq!(image[(1, 1, 1)], 0.8);
    assert_eq!(image[(1, 1, 2)], 0.7);

    assert_eq!(image[(0, 0, 0)], 0.1);
    assert_eq!(image[(0, 0, 1)], 0.2);
    assert_eq!(image[(0, 0, 2)], 0.3);
}
