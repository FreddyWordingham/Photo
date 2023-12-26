//! Instance builder structure.

use serde::{Deserialize, Serialize};

/// Builds an [`Entity`] instance.
#[derive(Deserialize, Serialize)]
pub struct EntityBuilder {
    /// Mesh identifier.
    mesh_id: String,
    /// Material identifier.
    material_id: String,
    /// Translation [x, y, z] (meters).
    translation: Option<[f64; 3]>,
    /// Rotation [x, y, z] (degrees).
    rotation: Option<[f64; 3]>,
    /// Scale factor.
    scale: Option<f64>,
}
