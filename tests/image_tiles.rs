use ndarray::{Array3, s};
use photo::{Channels, Image};

#[test]
fn test_view_tiles_separated() {
    let data = Array3::from_shape_fn((4, 6, 1), |(row, col, _)| (row * 6 + col) as u8);
    let image = Image::new(&data);

    let tiles = image.view_tiles((2, 3), (0, 0));
    assert_eq!(tiles.shape(), &[2, 2]);

    // Tile (0, 0) - Top left
    assert_eq!(tiles[[0, 0]].shape(), &[2, 3, 1]);
    assert_eq!(
        tiles[[0, 0]].slice(s![.., .., 0]),
        Array3::from_shape_vec((2, 3, 1), vec![0, 1, 2, 6, 7, 8])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (0, 1) - Top right
    assert_eq!(tiles[[0, 1]].shape(), &[2, 3, 1]);
    assert_eq!(
        tiles[[0, 1]].slice(s![.., .., 0]),
        Array3::from_shape_vec((2, 3, 1), vec![3, 4, 5, 9, 10, 11])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (1, 0) - Bottom left
    assert_eq!(tiles[[1, 0]].shape(), &[2, 3, 1]);
    assert_eq!(
        tiles[[1, 0]].slice(s![.., .., 0]),
        Array3::from_shape_vec((2, 3, 1), vec![12, 13, 14, 18, 19, 20])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (1, 1) - Bottom right
    assert_eq!(tiles[[1, 1]].shape(), &[2, 3, 1]);
    assert_eq!(
        tiles[[1, 1]].slice(s![.., .., 0]),
        Array3::from_shape_vec((2, 3, 1), vec![15, 16, 17, 21, 22, 23])
            .unwrap()
            .slice(s![.., .., 0])
    );
}

#[test]
fn test_view_tiles_with_overlap() {
    let data = Array3::from_shape_fn((5, 7, 1), |(row, col, _)| (row * 7 + col) as u8);
    let image = Image::new(&data);

    let tiles = image.view_tiles((3, 4), (1, 1));
    assert_eq!(tiles.shape(), &[2, 2]); // 2x2 grid of tiles

    // Tile (0, 0) - Top left
    assert_eq!(tiles[[0, 0]].shape(), &[3, 4, 1]);
    assert_eq!(
        tiles[[0, 0]].slice(s![.., .., 0]),
        Array3::from_shape_vec((3, 4, 1), vec![0, 1, 2, 3, 7, 8, 9, 10, 14, 15, 16, 17])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (0, 1) - Top right
    assert_eq!(tiles[[0, 1]].shape(), &[3, 4, 1]);
    assert_eq!(
        tiles[[0, 1]].slice(s![.., .., 0]),
        Array3::from_shape_vec((3, 4, 1), vec![3, 4, 5, 6, 10, 11, 12, 13, 17, 18, 19, 20])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (1, 0) - Bottom left
    assert_eq!(tiles[[1, 0]].shape(), &[3, 4, 1]);
    assert_eq!(
        tiles[[1, 0]].slice(s![.., .., 0]),
        Array3::from_shape_vec((3, 4, 1), vec![14, 15, 16, 17, 21, 22, 23, 24, 28, 29, 30, 31])
            .unwrap()
            .slice(s![.., .., 0])
    );
    // Tile (1, 1) - Bottom right
    assert_eq!(tiles[[1, 1]].shape(), &[3, 4, 1]);
    assert_eq!(
        tiles[[1, 1]].slice(s![.., .., 0]),
        Array3::from_shape_vec((3, 4, 1), vec![17, 18, 19, 20, 24, 25, 26, 27, 31, 32, 33, 34])
            .unwrap()
            .slice(s![.., .., 0])
    );
}

#[test]
#[should_panic(expected = "Tile height must be positive")]
fn test_view_tiles_zero_tile_height() {
    let data = Array3::from_shape_fn((4, 4, 1), |(row, col, _)| (row * 4 + col) as u8);
    let image = Image::new(&data);

    let _ = image.view_tiles((0, 2), (0, 0));
}

#[test]
#[should_panic(expected = "Overlap height must be less than tile height")]
fn test_view_tiles_invalid_overlap() {
    let data = Array3::from_shape_fn((4, 4, 1), |(row, col, _)| (row * 4 + col) as u8);
    let image = Image::new(&data);

    let _ = image.view_tiles((2, 2), (2, 0));
}

#[test]
#[should_panic(expected = "Image must contain an integer number of tiles in the vertical direction")]
fn test_view_tiles_non_integer_tiles_vertical() {
    let data = Array3::from_shape_fn((5, 4, 1), |(row, col, _)| (row * 4 + col) as u8);
    let image = Image::new(&data);

    let _ = image.view_tiles((2, 2), (0, 0));
}

#[test]
fn test_copy_tiles_separated() {
    let data = Array3::<u8>::from_shape_fn((6, 8, 1), |(row, col, _)| (row * 10 + col) as u8);
    let image = Image::new(&data);

    let tiles = image.copy_tiles((3, 4), (0, 0));
    assert_eq!(tiles.dim(), (2, 2));

    for tile_row in 0..2 {
        for tile_col in 0..2 {
            let tile = &tiles[(tile_row, tile_col)];

            assert_eq!(tile.format(), Channels::Grey);
            assert_eq!(tile.resolution(), (3, 4));

            for row in 0..3 {
                for col in 0..4 {
                    let orig_row = tile_row * 3 + row;
                    let orig_col = tile_col * 4 + col;

                    assert_eq!(tile[(row, col, 0)], image[(orig_row, orig_col, 0)]);
                }
            }
        }
    }
}

#[test]
fn test_copy_tiles_overlapping() {
    let data = Array3::<u8>::from_shape_fn((4, 7, 1), |(row, col, _)| (row * 10 + col) as u8);
    let image = Image::new(&data);

    let tiles = image.copy_tiles((3, 4), (2, 1));
    assert_eq!(tiles.dim(), (2, 2));

    let first_tile = &tiles[(0, 0)];
    assert_eq!(first_tile.resolution(), (3, 4));
    for row in 0..3 {
        for col in 0..4 {
            assert_eq!(first_tile[(row, col, 0)], image[(row, col, 0)]);
        }
    }

    let second_tile = &tiles[(0, 1)];
    assert_eq!(second_tile.resolution(), (3, 4));
    for row in 0..3 {
        for col in 0..4 {
            assert_eq!(second_tile[(row, col, 0)], image[(row, col + 3, 0)]);
        }
    }
}
