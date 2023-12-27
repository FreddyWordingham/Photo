//! Instance structure.

use nalgebra::{Point3, Similarity3};

use crate::{
    geometry::{Aabb, Mesh},
    world::Material,
};

/// Observable entity.
pub struct Entity<'a> {
    /// Base triangle mesh.
    mesh: &'a Mesh,
    /// Surface material.
    material: &'a Material<'a>,
    /// Transformation matrix.
    transformation: Similarity3<f64>,
    /// Inverse transformation matrix.
    inverse_transformation: Similarity3<f64>,
    /// Bounding box.
    aabb: Aabb,
}

impl<'a> Entity<'a> {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(
        mesh: &'a Mesh,
        material: &'a Material<'a>,
        transformation: Similarity3<f64>,
    ) -> Self {
        Self {
            mesh,
            material,
            transformation,
            inverse_transformation: transformation.inverse(),
            aabb: Self::init_aabb(mesh, &transformation),
        }
    }

    /// Initialise the [`Entity`]'s bounding box.
    fn init_aabb(mesh: &Mesh, transformation: &Similarity3<f64>) -> Aabb {
        let mut mins = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut maxs = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for triangle in mesh.triangles() {
            for vertex in triangle.vertex_positions() {
                let transformed_vertex = transformation * vertex;
                mins = Point3::new(
                    mins.x.min(transformed_vertex.x),
                    mins.y.min(transformed_vertex.y),
                    mins.z.min(transformed_vertex.z),
                );
                maxs = Point3::new(
                    maxs.x.max(transformed_vertex.x),
                    maxs.y.max(transformed_vertex.y),
                    maxs.z.max(transformed_vertex.z),
                );
            }
        }

        Aabb::new(mins, maxs)
    }
}
