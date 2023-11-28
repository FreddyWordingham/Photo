use crate::assets::Gradient;

pub enum Material {
    Diffuse {
        colour: Gradient,
    },
    Reflective {
        colour: Gradient,
        absorption: f64,
    },
    Refractive {
        colour: Gradient,
        absorption: f64,
        refractive_index: f64,
    },
}

impl Material {
    pub fn opacity(&self) -> f64 {
        match self {
            Self::Diffuse { .. } => 1.0,
            Self::Reflective { absorption, .. } => *absorption,
            Self::Refractive { absorption, .. } => *absorption,
        }
    }
}
