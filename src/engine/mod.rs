//! Rendering engine module.

mod ambient;
mod diffuse;
mod distance;
mod full;
mod normal;
mod side;
mod stencil;

pub use ambient::ambient;
pub use diffuse::diffuse;
pub use distance::distance;
pub use full::full;
pub use normal::normal;
pub use side::side;
pub use stencil::stencil;

use palette::LinSrgba;

use crate::{geometry::Ray, render::Settings, world::Scene};

pub type Engine = Box<dyn Fn(&Settings, &Scene, Ray) -> LinSrgba + Send + Sync>;
