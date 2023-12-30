//! Rendering engine module.

mod stencil;

pub use stencil::stencil;

use crate::{geometry::Ray, render::Sample, world::Scene};

pub type Engine = Box<dyn Fn(&Scene, [usize; 2], &Ray) -> Sample + Send + Sync>;
