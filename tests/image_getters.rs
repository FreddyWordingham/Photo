use photo::Image;

#[test]
fn test_get_channel() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    let green = image.get_channel(1);
    assert!(green.iter().all(|&v| v == 0.2));
}

#[test]
fn test_get_pixel() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    let pixel = image.get_pixel((1, 1));

    assert_eq!(pixel[0], 0.1);
    assert_eq!(pixel[1], 0.2);
    assert_eq!(pixel[2], 0.3);
}

#[test]
fn test_get_component() {
    let image = Image::<f32>::filled((3, 3), &[0.1, 0.2, 0.3]);

    assert_eq!(image[(0, 0, 0)], 0.1);
    assert_eq!(image[(0, 0, 1)], 0.2);
    assert_eq!(image[(0, 0, 2)], 0.3);

    assert_eq!(image[(1, 1, 0)], 0.1);
    assert_eq!(image[(1, 1, 1)], 0.2);
    assert_eq!(image[(1, 1, 2)], 0.3);
}
