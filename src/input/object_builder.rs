use nalgebra::Similarity3;
use serde::{Deserialize, Serialize};

use crate::world::{Mesh, Object};

/// Scene object parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectBuilder {
    /// Mesh id.
    pub mesh_id: String,
    /// Optional translation. [x, y, z] (meters)
    pub translation: Option<[f64; 3]>,
    /// Optional rotation. [x, y, z] (degrees)
    pub rotation: Option<[f64; 3]>,
    /// Optional scale.
    pub scale: Option<f64>,
}

impl ObjectBuilder {
    /// Construct a new instance.
    pub fn new(
        mesh_id: String,
        translation: Option<[f64; 3]>,
        rotation: Option<[f64; 3]>,
        scale: Option<f64>,
    ) -> Self {
        let object_builder = Self {
            mesh_id,
            translation,
            rotation,
            scale,
        };

        debug_assert!(object_builder.is_valid());

        object_builder
    }

    /// Check that the current combination of values are valid.
    pub fn is_valid(&self) -> bool {
        if self.mesh_id.is_empty() {
            println!("INVALID! Invalid mesh id: {}", self.mesh_id);
            return false;
        }

        if let Some(translation) = &self.translation {
            if translation.iter().any(|&t| t.is_nan()) {
                println!(
                    "INVALID! Invalid translation: {:?} -> {:?}",
                    self.mesh_id, translation
                );
                return false;
            }
        }

        if let Some(rotation) = &self.rotation {
            if rotation.iter().any(|&r| r.is_nan()) {
                println!(
                    "INVALID! Invalid rotation: {:?} -> {:?}",
                    self.mesh_id, rotation
                );
                return false;
            }
        }

        if let Some(scale) = &self.scale {
            if *scale <= 0.0 {
                println!("INVALID! Invalid scale: {:?} -> {:?}", self.mesh_id, scale);
                return false;
            }
        }

        true
    }

    /// Build a renderable object.
    pub fn build(&self, mesh: &Mesh) -> Object {
        debug_assert!(self.is_valid());

        let translation = if let Some(translation) = &self.translation {
            nalgebra::Translation {
                vector: nalgebra::Vector3::new(translation[0], translation[1], translation[2]),
            }
        } else {
            nalgebra::Translation {
                vector: nalgebra::Vector3::new(0.0, 0.0, 0.0),
            }
        };
        println!("Translation: {:?}", translation);

        let rotation = if let Some(rotation) = &self.rotation {
            nalgebra::UnitQuaternion::from_euler_angles(
                rotation[0].to_radians(),
                rotation[1].to_radians(),
                rotation[2].to_radians(),
            )
        } else {
            nalgebra::UnitQuaternion::identity()
        };

        let scale = if let Some(scale) = &self.scale {
            *scale
        } else {
            1.0
        };

        let transformation = Similarity3::from_parts(translation, rotation, scale);

        return Object::new(self.mesh_id.clone(), mesh, transformation);
    }
}
