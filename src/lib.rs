pub mod run;
pub mod util;

pub use run::run;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn wasm_main() {
    let scale = 0.5;
    run((1920.0 * scale, 1080.0 * scale));
}
