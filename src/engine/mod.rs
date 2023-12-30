//! Rendering engine module.

mod ambient;
mod diffuse;
mod distance;
mod normal;
mod side;
mod stencil;

pub use ambient::ambient;
pub use diffuse::diffuse;
pub use distance::distance;
pub use normal::normal;
pub use side::side;
pub use stencil::stencil;

use crate::{geometry::Ray, render::Sample, world::Scene};

pub type Engine = Box<dyn Fn(&Scene, [usize; 2], &Ray) -> Sample + Send + Sync>;
