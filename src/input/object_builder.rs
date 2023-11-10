use serde::{Deserialize, Serialize};

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
