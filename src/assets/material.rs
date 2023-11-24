use crate::assets::Gradient;

pub enum Material {
    Diffuse {
        colour: Gradient,
    },
    Reflective {
        colour: Gradient,
        reflectivity: f32,
    },
    Refractive {
        colour: Gradient,
        refractive_index: f32,
    },
}
