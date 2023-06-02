use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    resolution: (u32, u32),
}

fn main() {
    println!("Hello, world!");

    let json_str = r#"
        {
            "resolution": [1920, 1080]
        }
    "#;

    let p: Parameters = serde_json::from_str(json_str).unwrap();
    println!("resolution: {}x{}", p.resolution.0, p.resolution.1);
}
