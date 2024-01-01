//! Post-processing effects module.

mod outline;

pub use outline::outline;

use crate::render::Tile;

pub type Effect = Box<dyn Fn(Tile) -> Tile>;
