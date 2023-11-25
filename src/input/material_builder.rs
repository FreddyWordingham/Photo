use serde::{Deserialize, Serialize};

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
}
