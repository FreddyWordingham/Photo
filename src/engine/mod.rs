//! Rendering engine module.

mod ambient;
mod diffuse;
mod distance;
mod normal;
mod reflective;
mod side;
mod stencil;

pub use ambient::ambient;
pub use diffuse::diffuse;
pub use distance::distance;
pub use normal::normal;
pub use reflective::reflective;
pub use side::side;
pub use stencil::stencil;

use crate::{
    geometry::Ray,
    render::{Sample, Settings},
    world::Scene,
};

pub type Engine = Box<dyn Fn(&Settings, &Scene, [usize; 2], Ray) -> Sample + Send + Sync>;
