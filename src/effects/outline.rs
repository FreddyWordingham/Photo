//! Outline effect.

use palette::LinSrgba;

use crate::render::Tile;

/// Colour all colours with a different adjacent colour black.
#[must_use]
#[inline]
pub fn outline(tile: Tile) -> Tile {
    let shape = tile.samples.shape();
    let num_rows = shape[0];
    let num_cols = shape[1];

    let mut new_tile = Tile::new(tile.tile_index, [num_rows, num_cols]);

    for row in 0..num_rows {
        for col in 0..num_cols {
            let current_colour = tile.samples[[row, col]].colour;

            let adjacent_colours = [
                row.checked_sub(1).map(|r| tile.samples[[r, col]].colour),
                (row + 1 < num_rows).then(|| tile.samples[[row + 1, col]].colour),
                col.checked_sub(1).map(|c| tile.samples[[row, c]].colour),
                (col + 1 < num_cols).then(|| tile.samples[[row, col + 1]].colour),
            ];

            if adjacent_colours
                .iter()
                .any(|&adj_colour| adj_colour.map_or(false, |c| c != current_colour))
            {
                new_tile.samples[[row, col]].colour = LinSrgba::new(0.0, 0.0, 0.0, 1.0);
            }
        }
    }

    new_tile
}
