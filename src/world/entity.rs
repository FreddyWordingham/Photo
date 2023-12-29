//! Instance structure.

use nalgebra::{Point3, Similarity3, Unit};

use crate::{
    geometry::{Aabb, Bounded, Mesh, Ray},
    render::Contact,
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
    #[must_use]
    #[inline]
    fn init_aabb(mesh: &Mesh, transformation: &Similarity3<f64>) -> Aabb {
        let (mins, maxs) = mesh
            .triangles()
            .flat_map(|triangle| *triangle.vertex_positions())
            .map(|vertex| transformation * vertex)
            .fold(
                (
                    Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
                    Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
                ),
                |(acc_mins, acc_maxs), transformed_vertex| {
                    (
                        Point3::new(
                            acc_mins.x.min(transformed_vertex.x),
                            acc_mins.y.min(transformed_vertex.y),
                            acc_mins.z.min(transformed_vertex.z),
                        ),
                        Point3::new(
                            acc_maxs.x.max(transformed_vertex.x),
                            acc_maxs.y.max(transformed_vertex.y),
                            acc_maxs.z.max(transformed_vertex.z),
                        ),
                    )
                },
            );

        Aabb::new(mins, maxs)
    }

    /// Test for an intersection distance with a [`Ray`].
    #[must_use]
    #[inline]
    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        let transformed_ray = ray * &self.inverse_transformation;
        self.mesh.ray_intersect(&transformed_ray)
    }

    /// Test for an intersection [`Ray`],
    /// return the distance to the intersection point, if one exists.
    #[must_use]
    #[inline]
    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        let transformed_ray = ray * &self.inverse_transformation;
        self.mesh
            .ray_intersect_distance(&transformed_ray)
            .map(|distance| distance * self.transformation.scaling())
    }

    /// Test for an intersection with a [`Ray`],
    /// return the properties of the [`Contact`] point, if one exists.
    #[must_use]
    #[inline]
    pub fn ray_intersect_hit(&self, ray: &Ray) -> Option<Contact> {
        self.mesh
            .ray_intersect_distance_normals(&(ray * &self.inverse_transformation))
            .map(|(distance, normal, smooth_normal)| {
                let is_inside = (ray * &self.inverse_transformation)
                    .direction()
                    .dot(&normal)
                    > 0.0;
                Contact::new(
                    is_inside,
                    distance * self.transformation.scaling(),
                    Unit::new_normalize(self.transformation.transform_vector(&normal)),
                    Unit::new_normalize(self.transformation.transform_vector(&smooth_normal)),
                    self.material,
                )
            })
    }
}

impl<'a> Bounded for Entity<'a> {
    /// Get the [`Aabb`] encompassing the [`Entity`].
    #[must_use]
    #[inline]
    fn aabb(&self) -> Aabb {
        self.aabb.clone()
    }
}
