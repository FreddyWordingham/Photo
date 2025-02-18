use ndarray::{s, Array2, Array3};
use photo::ImageRGBA;
use rand::seq::IteratorRandom;
use std::collections::{HashSet, VecDeque};

const INPUT_DIR: &str = "input";
const TILE_SIZE: [usize; 2] = [14, 14];

fn main() {
    let image_name = "tileset2.png";
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

    let map = wave_function_collapse(&rules, [51, 51]);
    println!("{:?}", map);

    let output = render_image(&map, &unique_tiles);
    let image = ImageRGBA::new(output);
    println!("{}", image);
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

    // Determine the rules for each tile
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

pub fn wave_function_collapse(rules: &[Rules], resolution: [usize; 2]) -> Array2<usize> {
    const MAX_ATTEMPTS: usize = 100;
    for attempt in 0..MAX_ATTEMPTS {
        if let Some(map) = collapse_attempt(rules, resolution) {
            return map;
        }
        eprintln!("Attempt {} failed, retrying...", attempt + 1);
    }
    panic!(
        "Failed to generate a valid map after {} attempts",
        MAX_ATTEMPTS
    );
}

fn collapse_attempt(rules: &[Rules], resolution: [usize; 2]) -> Option<Array2<usize>> {
    let mut rng = rand::rng();
    let rows = resolution[0];
    let cols = resolution[1];
    let num_tiles = rules.len();

    // Each cell starts with all possible tile indices.
    let mut possibilities: Vec<Vec<HashSet<usize>>> =
        vec![vec![(0..num_tiles).collect(); cols]; rows];
    let mut queue = VecDeque::new();

    // Helper to enqueue valid coordinates.
    fn push_if_valid(
        queue: &mut VecDeque<(usize, usize)>,
        r: usize,
        c: usize,
        rows: usize,
        cols: usize,
    ) {
        if r < rows && c < cols {
            queue.push_back((r, c));
        }
    }

    loop {
        // Find the cell with the lowest entropy (>1 possibility).
        let mut min_entropy = usize::MAX;
        let mut min_pos = None;
        for r in 0..rows {
            for c in 0..cols {
                let len = possibilities[r][c].len();
                if len > 1 && len < min_entropy {
                    min_entropy = len;
                    min_pos = Some((r, c));
                }
            }
        }
        // All cells collapsed.
        if min_pos.is_none() {
            break;
        }
        let (r, c) = min_pos.unwrap();

        // Randomly pick one possibility.
        let chosen = *possibilities[r][c].iter().choose(&mut rng)?;
        possibilities[r][c] = [chosen].iter().cloned().collect();

        // Enqueue neighbors.
        if r > 0 {
            push_if_valid(&mut queue, r - 1, c, rows, cols);
        }
        if r < rows - 1 {
            push_if_valid(&mut queue, r + 1, c, rows, cols);
        }
        if c > 0 {
            push_if_valid(&mut queue, r, c - 1, rows, cols);
        }
        if c < cols - 1 {
            push_if_valid(&mut queue, r, c + 1, rows, cols);
        }

        // Propagate constraints.
        while let Some((i, j)) = queue.pop_front() {
            let mut new_set = possibilities[i][j].clone();

            // Above: allowed by the neighbor's south rule.
            if i > 0 {
                let mut allowed = HashSet::new();
                for &nbr_tile in &possibilities[i - 1][j] {
                    allowed.extend(rules[nbr_tile].south.iter().cloned());
                }
                new_set = new_set.intersection(&allowed).cloned().collect();
            }
            // Below: allowed by the neighbor's north rule.
            if i < rows - 1 {
                let mut allowed = HashSet::new();
                for &nbr_tile in &possibilities[i + 1][j] {
                    allowed.extend(rules[nbr_tile].north.iter().cloned());
                }
                new_set = new_set.intersection(&allowed).cloned().collect();
            }
            // Left: allowed by the neighbor's east rule.
            if j > 0 {
                let mut allowed = HashSet::new();
                for &nbr_tile in &possibilities[i][j - 1] {
                    allowed.extend(rules[nbr_tile].east.iter().cloned());
                }
                new_set = new_set.intersection(&allowed).cloned().collect();
            }
            // Right: allowed by the neighbor's west rule.
            if j < cols - 1 {
                let mut allowed = HashSet::new();
                for &nbr_tile in &possibilities[i][j + 1] {
                    allowed.extend(rules[nbr_tile].west.iter().cloned());
                }
                new_set = new_set.intersection(&allowed).cloned().collect();
            }

            if new_set.len() < possibilities[i][j].len() {
                possibilities[i][j] = new_set;
                if possibilities[i][j].is_empty() {
                    return None; // Contradiction.
                }
                if i > 0 {
                    push_if_valid(&mut queue, i - 1, j, rows, cols);
                }
                if i < rows - 1 {
                    push_if_valid(&mut queue, i + 1, j, rows, cols);
                }
                if j > 0 {
                    push_if_valid(&mut queue, i, j - 1, rows, cols);
                }
                if j < cols - 1 {
                    push_if_valid(&mut queue, i, j + 1, rows, cols);
                }
            }
        }
    }

    // Build final map ensuring every cell is collapsed.
    let mut result = Array2::<usize>::zeros((rows, cols));
    for i in 0..rows {
        for j in 0..cols {
            if possibilities[i][j].len() != 1 {
                return None;
            }
            result[[i, j]] = *possibilities[i][j].iter().next()?;
        }
    }
    Some(result)
}

pub fn render_image(map: &Array2<usize>, tiles: &[(ImageRGBA<u8>, usize)]) -> Array3<u8> {
    // All tiles are assumed to have the same dimensions.
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
            // Assumes ImageRGBA provides a method to convert to an ndarray view.
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
