//! Instance structure.

use nalgebra::Similarity3;

use crate::{
    geometry::{Aabb, Mesh},
    world::Material,
};

/// Observable entity.
pub struct Entity<'a> {
    /// Bounding box.
    aabb: Aabb,
    /// Base triangle mesh.
    mesh: &'a Mesh,
    /// Surface material.
    material: &'a Material<'a>,
    /// Transformation matrix.
    transformation: Similarity3<f64>,
    /// Inverse transformation matrix.
    inverse_transformation: Similarity3<f64>,
}
