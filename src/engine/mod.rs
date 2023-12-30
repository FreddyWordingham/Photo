//! Rendering engine module.

mod distance;
mod normal;
mod stencil;

pub use distance::distance;
pub use normal::normal;
pub use stencil::stencil;

use crate::{geometry::Ray, render::Sample, world::Scene};

pub type Engine = Box<dyn Fn(&Scene, [usize; 2], &Ray) -> Sample + Send + Sync>;
