use pollster::FutureExt;
use std::str::FromStr;

use photo::gpu::{Chunk, Hardware, Shaders};

fn main() {
    let mut chunks = read_input_chunks();
    println!(" Input: {:?}", chunks);

    let hardware = (Hardware::new()).block_on();
    let shaders = Shaders::new(
        &hardware,
        &chunks,
        vec![
            include_str!("invert_colour.wgsl"),
            include_str!("add_colour.wgsl"),
            include_str!("sub_colour.wgsl"),
        ],
    );

    chunks = shaders.run(&mut chunks, 2).block_on();
    println!("Output: {:?}", chunks);
}

fn read_input_chunks() -> Vec<Chunk> {
    if std::env::args().len() <= 1 {
        let default = vec![
            Chunk {
                col: [0.0, 1.0, 0.0, 1.0],
                x: 0.1,
                pad_a: 0.0,
                pad_b: 0.0,
                pad_c: 0.0,
            },
            Chunk {
                col: [1.0, 0.0, 1.0, 1.0],
                x: 0.2,
                pad_a: 0.0,
                pad_b: 0.0,
                pad_c: 0.0,
            },
        ];
        println!("No numbers were provided, defaulting to {default:?}");
        return default;
    }

    let elements: Vec<_> = std::env::args()
        .skip(1)
        .map(|s| f32::from_str(&s).expect("Invalid input"))
        .collect();

    elements
        .chunks_exact(5)
        .map(|chunk| Chunk {
            col: [chunk[0], chunk[1], chunk[2], chunk[3]],
            x: chunk[4],
            pad_a: 0.0,
            pad_b: 0.0,
            pad_c: 0.0,
        })
        .collect()
}
