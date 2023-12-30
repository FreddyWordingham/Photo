//! Rendering engine module.

mod ambient;
mod distance;
mod material;
mod normal;
mod stencil;

pub use ambient::ambient;
pub use distance::distance;
pub use material::material;
pub use normal::normal;
pub use stencil::stencil;

use crate::{geometry::Ray, render::Sample, world::Scene};

pub type Engine = Box<dyn Fn(&Scene, [usize; 2], &Ray) -> Sample + Send + Sync>;
