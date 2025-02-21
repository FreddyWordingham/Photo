use ndarray::{Array2, Array3, s};
use photo::ImageRGBA;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};

const INPUT_DIR: &str = "input";
const TILE_SIZE: [usize; 2] = [16, 16];

fn main() {
    let image_name = "villiage.png";
    let filepath = format!("{}/{}", INPUT_DIR, image_name);

    let image = ImageRGBA::<u8>::load(filepath).expect("Failed to load image");
    println!("{}", image);
    println!("Height {}", image.height());
    println!("Width {}", image.width());

    let image_tiles = image.tiles(TILE_SIZE);
    let unique_tiles = image.unique_tiles(TILE_SIZE);
    for (tile, _frequency) in &unique_tiles {
        println!("{}", tile);
    }

    let tile_mapping = create_tile_mapping(&image_tiles, &unique_tiles);
    println!("{:?}", tile_mapping);

    let rules = create_tile_rules(&tile_mapping);
    for rule in &rules {
        println!("{:?}", rule);
    }

    let map = wave_function_collapse_backtracking(&rules, [11, 31]);
    println!("{:?}", map);

    let output = render_image(&map, &unique_tiles);
    let image = ImageRGBA::new(output);
    // println!("{}", image);
    image.save("output/map.png").expect("Failed to save image");
}

fn create_tile_mapping(
    image_tiles: &Array2<ImageRGBA<u8>>,
    unique_tiles: &[(ImageRGBA<u8>, usize)],
) -> Array2<usize> {
    let mut tile_mapping = Array2::<usize>::zeros(image_tiles.dim());
    for (mut map_index, tile) in tile_mapping.iter_mut().zip(image_tiles.iter()) {
        for (unique_tile_index, (unique_tile, _frequency)) in unique_tiles.iter().enumerate() {
            if tile == unique_tile {
                *map_index = unique_tile_index;
                break;
            }
        }
    }
    tile_mapping
}

#[derive(Debug, Default, Clone)]
struct Rules {
    north: HashSet<usize>,
    east: HashSet<usize>,
    south: HashSet<usize>,
    west: HashSet<usize>,
}

fn create_tile_rules(tile_mapping: &ndarray::Array2<usize>) -> Vec<Rules> {
    let height = tile_mapping.shape()[0];
    let width = tile_mapping.shape()[1];
    let max = *tile_mapping.iter().max().unwrap();
    let mut rules = vec![Rules::default(); max + 1];

    for (index, &t) in tile_mapping.indexed_iter() {
        if index.0 > 0 {
            rules[t].north.insert(tile_mapping[[index.0 - 1, index.1]]);
        }
        if index.0 < height - 1 {
            rules[t].south.insert(tile_mapping[[index.0 + 1, index.1]]);
        }
        if index.1 > 0 {
            rules[t].west.insert(tile_mapping[[index.0, index.1 - 1]]);
        }
        if index.1 < width - 1 {
            rules[t].east.insert(tile_mapping[[index.0, index.1 + 1]]);
        }
    }
    rules
}

pub fn wave_function_collapse_backtracking(
    rules: &[Rules],
    resolution: [usize; 2],
) -> Array2<usize> {
    let (rows, cols) = (resolution[0], resolution[1]);
    let num_tiles = rules.len();
    // Each cell starts with all tile indices.
    let mut possibilities: Vec<Vec<HashSet<usize>>> =
        vec![vec![(0..num_tiles).collect(); cols]; rows];
    if let Some(solution) = backtrack(&mut possibilities, rules, rows, cols) {
        solution
    } else {
        panic!("No valid solution found");
    }
}

fn backtrack(
    possibilities: &mut Vec<Vec<HashSet<usize>>>,
    rules: &[Rules],
    rows: usize,
    cols: usize,
) -> Option<Array2<usize>> {
    // If every cell is collapsed, build and return the solution.
    if possibilities
        .iter()
        .all(|row| row.iter().all(|cell| cell.len() == 1))
    {
        let mut result = Array2::<usize>::zeros((rows, cols));
        for r in 0..rows {
            for c in 0..cols {
                result[[r, c]] = *possibilities[r][c].iter().next().unwrap();
            }
        }
        return Some(result);
    }

    // Find the cell with the fewest possibilities (>1).
    let mut min_options = usize::MAX;
    let mut min_cell = (0, 0);
    for r in 0..rows {
        for c in 0..cols {
            let len = possibilities[r][c].len();
            if len > 1 && len < min_options {
                min_options = len;
                min_cell = (r, c);
            }
        }
    }
    let (r, c) = min_cell;
    let options: Vec<usize> = possibilities[r][c].iter().cloned().collect();

    // Try each candidate for this cell.
    for option in options {
        let mut new_state = possibilities.clone();
        new_state[r][c] = std::iter::once(option).collect();

        // Propagate constraints from this decision.
        if let Some(propagated) = propagate(&mut new_state, rules, rows, cols) {
            if let Some(solution) = backtrack(&mut propagated.clone(), rules, rows, cols) {
                return Some(solution);
            }
        }
    }
    None
}

fn propagate(
    possibilities: &mut Vec<Vec<HashSet<usize>>>,
    rules: &[Rules],
    rows: usize,
    cols: usize,
) -> Option<Vec<Vec<HashSet<usize>>>> {
    let mut queue = VecDeque::new();
    // Enqueue all collapsed cells.
    for r in 0..rows {
        for c in 0..cols {
            if possibilities[r][c].len() == 1 {
                queue.push_back((r, c));
            }
        }
    }

    while let Some((r, c)) = queue.pop_front() {
        for (nr, nc) in neighbors(r, c, rows, cols) {
            let current = possibilities[nr][nc].clone();
            let mut allowed = current.clone();

            // Refine based on the neighbor's constraints.
            if nr > 0 {
                let valid: HashSet<_> = possibilities[nr - 1][nc]
                    .iter()
                    .flat_map(|&t| rules[t].south.iter().cloned())
                    .collect();
                allowed = allowed.intersection(&valid).cloned().collect();
            }
            if nr < rows - 1 {
                let valid: HashSet<_> = possibilities[nr + 1][nc]
                    .iter()
                    .flat_map(|&t| rules[t].north.iter().cloned())
                    .collect();
                allowed = allowed.intersection(&valid).cloned().collect();
            }
            if nc > 0 {
                let valid: HashSet<_> = possibilities[nr][nc - 1]
                    .iter()
                    .flat_map(|&t| rules[t].east.iter().cloned())
                    .collect();
                allowed = allowed.intersection(&valid).cloned().collect();
            }
            if nc < cols - 1 {
                let valid: HashSet<_> = possibilities[nr][nc + 1]
                    .iter()
                    .flat_map(|&t| rules[t].west.iter().cloned())
                    .collect();
                allowed = allowed.intersection(&valid).cloned().collect();
            }
            if allowed.is_empty() {
                return None; // Contradiction.
            }
            if allowed.len() < current.len() {
                possibilities[nr][nc] = allowed;
                queue.push_back((nr, nc));
            }
        }
    }
    Some(possibilities.clone())
}

fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut nbrs = Vec::with_capacity(4);
    if r > 0 {
        nbrs.push((r - 1, c));
    }
    if r < rows - 1 {
        nbrs.push((r + 1, c));
    }
    if c > 0 {
        nbrs.push((r, c - 1));
    }
    if c < cols - 1 {
        nbrs.push((r, c + 1));
    }
    nbrs
}

pub fn render_image(map: &Array2<usize>, tiles: &[(ImageRGBA<u8>, usize)]) -> Array3<u8> {
    let tile_height = tiles[0].0.height();
    let tile_width = tiles[0].0.width();

    let (map_rows, map_cols) = map.dim();
    let channels = 4; // RGBA

    let output_height = map_rows * tile_height;
    let output_width = map_cols * tile_width;
    let mut output = Array3::<u8>::zeros((output_height, output_width, channels));

    for (i, row) in map.outer_iter().enumerate() {
        for (j, &tile_idx) in row.iter().enumerate() {
            let tile = &tiles[tile_idx].0;
            let tile_data = &tile.data;
            let y_offset = i * tile_height;
            let x_offset = j * tile_width;
            output
                .slice_mut(s![
                    y_offset..y_offset + tile_height,
                    x_offset..x_offset + tile_width,
                    ..
                ])
                .assign(&tile_data);
        }
    }
    output
}
