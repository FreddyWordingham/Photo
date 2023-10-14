use pollster::FutureExt;
use std::str::FromStr;

use photo::gpu::{ComputeShaderRunner, Hardware};
use photo::Settings;

fn main() {
    let mut chunks = read_input_chunks();
    println!(" Input: {:?}", chunks);

    let settings = Settings { x: 0.0, y: 0.0 };

    let hardware = (Hardware::new()).block_on();
    let shaders = ComputeShaderRunner::new(&hardware, include_str!("add.wgsl"), settings, &chunks);

    chunks = shaders.run(&settings, &mut chunks).block_on();
    println!("Output: {:?}", chunks);

    let settings = Settings { x: 0.1, y: 0.1 };

    chunks = shaders.run(&settings, &mut chunks).block_on();
    println!("Output: {:?}", chunks);
}

fn read_input_chunks() -> Vec<[f32; 4]> {
    if std::env::args().len() <= 1 {
        let default = vec![[0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]];
        println!("No numbers were provided, defaulting to {default:?}");
        return default;
    }

    let elements: Vec<_> = std::env::args()
        .skip(1)
        .map(|s| f32::from_str(&s).expect("Invalid input"))
        .collect();

    elements
        .chunks_exact(4)
        .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
        .collect()
}
