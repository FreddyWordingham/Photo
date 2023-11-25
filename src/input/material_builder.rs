use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::assets::{Gradient, Material};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialBuilder {
    Diffuse {
        gradient_id: String,
    },
    Reflective {
        gradient_id: String,
        reflectivity: f32,
    },
    Refractive {
        gradient_id: String,
        refractive_index: f32,
    },
}

impl MaterialBuilder {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Diffuse { gradient_id } => !gradient_id.is_empty(),
            Self::Reflective {
                gradient_id,
                reflectivity,
            } => !gradient_id.is_empty() && reflectivity.is_finite() && reflectivity >= &0.0,
            Self::Refractive {
                gradient_id,
                refractive_index,
            } => !gradient_id.is_empty() && refractive_index.is_finite() && refractive_index > &0.0,
        }
    }

    pub fn gradient_ids(&self) -> Vec<&str> {
        match self {
            Self::Diffuse { gradient_id } => vec![gradient_id],
            Self::Reflective { gradient_id, .. } => vec![gradient_id],
            Self::Refractive { gradient_id, .. } => vec![gradient_id],
        }
    }

    pub fn build(&self, gradients: &HashMap<String, Gradient>) -> Material {
        match self {
            Self::Diffuse { gradient_id } => Material::Diffuse {
                colour: gradients[gradient_id].clone(),
            },
            Self::Reflective {
                gradient_id,
                reflectivity,
            } => Material::Reflective {
                colour: gradients[gradient_id].clone(),
                reflectivity: *reflectivity,
            },
            Self::Refractive {
                gradient_id,
                refractive_index,
            } => Material::Refractive {
                colour: gradients[gradient_id].clone(),
                refractive_index: *refractive_index,
            },
        }
    }
}
