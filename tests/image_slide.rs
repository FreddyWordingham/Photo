use ndarray::Array3;
use photo::Image;

#[test]
fn test_copy_slide() {
    let data = Array3::<u8>::from_shape_fn((4, 4, 1), |(i, j, _)| (i * 10 + j) as u8);
    let image = Image::new(&data);

    let slid_image = image.copy_slide((1, 2));
    assert_eq!(slid_image.resolution(), (4, 4));

    // Original values in a 4x4 image:
    // 00 01 02 03
    // 10 11 12 13
    // 20 21 22 23
    // 30 31 32 33

    // After sliding by (1, 2), the values should be:
    // 32 33 30 31
    // 02 03 00 01
    // 12 13 10 11
    // 22 23 20 21

    assert_eq!(slid_image[(0, 0, 0)], 32);
    assert_eq!(slid_image[(0, 1, 0)], 33);
    assert_eq!(slid_image[(0, 2, 0)], 30);
    assert_eq!(slid_image[(0, 3, 0)], 31);

    assert_eq!(slid_image[(1, 0, 0)], 02);
    assert_eq!(slid_image[(1, 1, 0)], 03);
    assert_eq!(slid_image[(1, 2, 0)], 00);
    assert_eq!(slid_image[(1, 3, 0)], 01);

    assert_eq!(slid_image[(2, 0, 0)], 12);
    assert_eq!(slid_image[(2, 1, 0)], 13);
    assert_eq!(slid_image[(2, 2, 0)], 10);
    assert_eq!(slid_image[(2, 3, 0)], 11);

    assert_eq!(slid_image[(3, 0, 0)], 22);
    assert_eq!(slid_image[(3, 1, 0)], 23);
    assert_eq!(slid_image[(3, 2, 0)], 20);
    assert_eq!(slid_image[(3, 3, 0)], 21);

    // // Test in-place sliding
    // let mut mutable_image = image.clone();
    // mutable_image.slide_inplace((1, 2));

    // // Check that in-place sliding gives the same result as copy_slide
    // for i in 0..4 {
    //     for j in 0..4 {
    //         assert_eq!(mutable_image[(i, j, 0)], slid_image[(i, j, 0)]);
    //     }
    // }
}

#[test]
fn test_inplace_slide() {
    let data = Array3::<u8>::from_shape_fn((4, 4, 1), |(i, j, _)| (i * 10 + j) as u8);
    let mut image = Image::new(&data);

    image.slide_inplace((1, 2));
    assert_eq!(image.resolution(), (4, 4));

    // Original values in a 4x4 image:
    // 00 01 02 03
    // 10 11 12 13
    // 20 21 22 23
    // 30 31 32 33

    // After sliding by (1, 2), the values should be:
    // 32 33 30 31
    // 02 03 00 01
    // 12 13 10 11
    // 22 23 20 21

    assert_eq!(image[(0, 0, 0)], 32);
    assert_eq!(image[(0, 1, 0)], 33);
    assert_eq!(image[(0, 2, 0)], 30);
    assert_eq!(image[(0, 3, 0)], 31);

    assert_eq!(image[(1, 0, 0)], 02);
    assert_eq!(image[(1, 1, 0)], 03);
    assert_eq!(image[(1, 2, 0)], 00);
    assert_eq!(image[(1, 3, 0)], 01);

    assert_eq!(image[(2, 0, 0)], 12);
    assert_eq!(image[(2, 1, 0)], 13);
    assert_eq!(image[(2, 2, 0)], 10);
    assert_eq!(image[(2, 3, 0)], 11);

    assert_eq!(image[(3, 0, 0)], 22);
    assert_eq!(image[(3, 1, 0)], 23);
    assert_eq!(image[(3, 2, 0)], 20);
    assert_eq!(image[(3, 3, 0)], 21);
}
