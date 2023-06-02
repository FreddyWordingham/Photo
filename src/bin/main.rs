use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Set a custom int
    #[structopt(short = "i", long = "integer")]
    num: i32,

    /// Set a custom boolean
    #[structopt(short = "b", long = "boolean")]
    flag: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    resolution: (u32, u32),
}

fn main() {
    let args = Cli::from_args();
    println!("num: {}, flag: {}", args.num, args.flag);

    let json_str = r#"
        {
            "resolution": [1920, 1080]
        }
    "#;

    let p: Parameters = serde_json::from_str(json_str).unwrap();
    println!("resolution: {}x{}", p.resolution.0, p.resolution.1);
}
