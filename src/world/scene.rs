//! Scene collection structure.

use crate::{
    geometry::Bvh,
    world::{Entity, Light},
};

/// Collection of entities and lights.
pub struct Scene<'a> {
    /// Lights in the scene.
    lights: Vec<Light>,
    /// Entities in the scene.
    entities: Vec<Entity<'a>>,
    /// Bounding Volume Hierarchy.
    bvh: Bvh,
}
