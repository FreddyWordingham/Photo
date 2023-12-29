//! Smooth triangle structure.

use core::f64::EPSILON;

use nalgebra::{Point3, Unit, Vector3};

use crate::geometry::{Aabb, Bounded, Ray};

/// Three-dimensional triangle with interpolated surface normals.
pub struct Triangle {
    /// Vertex positions.
    vertex_positions: [Point3<f64>; 3],
    /// Vertex normals.
    vertex_normals: [Unit<Vector3<f64>>; 3],
}

impl Triangle {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(
        vertex_positions: [Point3<f64>; 3],
        vertex_normals: [Unit<Vector3<f64>>; 3],
    ) -> Self {
        Self {
            vertex_positions,
            vertex_normals,
        }
    }

    /// Access the vertex positions.
    #[must_use]
    #[inline]
    pub const fn vertex_positions(&self) -> &[Point3<f64>; 3] {
        &self.vertex_positions
    }

    /// Access the vertex normals.
    #[must_use]
    #[inline]
    pub const fn vertex_normals(&self) -> &[Unit<Vector3<f64>>; 3] {
        &self.vertex_normals
    }

    /// Calculate the surface normal of the triangle.
    #[must_use]
    #[inline]
    fn plane_normal(&self) -> Unit<Vector3<f64>> {
        let edge_u = self.vertex_positions[1] - self.vertex_positions[0];
        let edge_v = self.vertex_positions[2] - self.vertex_positions[0];
        Unit::new_normalize(edge_u.cross(&edge_v))
    }

    /// Calculate an edge vector of the triangle.
    #[must_use]
    #[inline]
    fn edge_axis(&self, index: usize) -> Unit<Vector3<f64>> {
        debug_assert!(index < 3, "Triangle edge index must be less than 3!");

        Unit::new_normalize(self.vertex_positions[(index + 1) % 3] - self.vertex_positions[index])
    }

    /// Project the [`Triangle`] onto the given axis.
    #[must_use]
    #[inline]
    fn project_onto_axis(&self, axis: &Unit<Vector3<f64>>) -> (f64, f64) {
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for &vertex in &self.vertex_positions {
            let projection = vertex.coords.dot(axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }

    /// Check if the volume of the [`Aabb`] overlaps with the volume of the [`Triangle`].
    #[must_use]
    #[inline]
    fn overlap(&self, aabb: &Aabb) -> bool {
        if !triangle_overlaps_aabb_on_box_axes(self, aabb) {
            return false;
        }

        let normal = self.plane_normal();
        if !triangle_overlaps_aabb_on_axis(self, aabb, &normal) {
            return false;
        }

        let box_axes = [
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        ];

        if !(0..3)
            .flat_map(|i| box_axes.iter().map(move |box_axis| (i, box_axis)))
            .all(|(i, box_axis)| {
                let axis = Unit::new_normalize(self.edge_axis(i).cross(box_axis));
                triangle_overlaps_aabb_on_axis(self, aabb, &axis)
            })
        {
            return false;
        }

        true
    }

    /// Test for an intersection distance with a [`Ray`].
    #[must_use]
    #[inline]
    #[allow(clippy::many_single_char_names, clippy::min_ident_chars)]
    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        let edge1 = self.vertex_positions[1] - self.vertex_positions[0];
        let edge2 = self.vertex_positions[2] - self.vertex_positions[0];
        let h = ray.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertex_positions[0];
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * edge2.dot(&q);

        t > EPSILON
    }

    /// Test for an intersection [`Ray`],
    /// returning the distance to the intersection point, if one exists.
    #[must_use]
    #[inline]
    #[allow(clippy::many_single_char_names, clippy::min_ident_chars)]
    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        let edge1 = self.vertex_positions[1] - self.vertex_positions[0];
        let edge2 = self.vertex_positions[2] - self.vertex_positions[0];
        let h = ray.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertex_positions[0];
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > EPSILON {
            return Some(t);
        }

        None
    }

    /// Test for an intersection [`Ray`],
    /// returning the distance, plane normal and interpolated normal at the intersection point, if one exists.
    #[must_use]
    #[inline]
    #[allow(
        clippy::complexity,
        clippy::many_single_char_names,
        clippy::min_ident_chars
    )]
    pub fn ray_intersect_distance_normals(
        &self,
        ray: &Ray,
    ) -> Option<(f64, Unit<Vector3<f64>>, Unit<Vector3<f64>>)> {
        let edge1 = self.vertex_positions[1] - self.vertex_positions[0];
        let edge2 = self.vertex_positions[2] - self.vertex_positions[0];
        let h = ray.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.vertex_positions[0];
        let u = f * s.dot(&h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > EPSILON {
            let w = 1.0 - u - v;
            return Some((
                t,
                self.plane_normal(),
                Unit::new_normalize(
                    w * self.vertex_normals[0].as_ref()
                        + u * self.vertex_normals[1].as_ref()
                        + v * self.vertex_normals[2].as_ref(),
                ),
            ));
        }

        None
    }
}

impl Bounded for Triangle {
    /// Get the [`Aabb`] encompassing the [`Triangle`].
    #[must_use]
    #[inline]
    fn aabb(&self) -> Aabb {
        let mut mins = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut maxs = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for vertex in &self.vertex_positions {
            mins = Point3::new(
                mins.x.min(vertex.x),
                mins.y.min(vertex.y),
                mins.z.min(vertex.z),
            );
            maxs = Point3::new(
                maxs.x.max(vertex.x),
                maxs.y.max(vertex.y),
                maxs.z.max(vertex.z),
            );
        }

        Aabb::new(mins, maxs)
    }
}

/// Determine if a triangle overlaps an axis-aligned bounding box on the Cartesian axes.
fn triangle_overlaps_aabb_on_box_axes(triangle: &Triangle, aabb: &Aabb) -> bool {
    let axes = [Vector3::x_axis(), Vector3::y_axis(), Vector3::z_axis()];

    if !axes.iter().all(|axis| {
        // Project both the triangle and the AABB onto the axis
        let (min_tri, max_tri) = triangle.project_onto_axis(axis);
        let (min_aabb, max_aabb) = aabb.project_onto_axis(axis);

        // Check for overlap
        !(max_tri < min_aabb || min_tri > max_aabb)
    }) {
        return false;
    }

    true
}

/// Determine if a triangle overlaps an axis-aligned bounding box on the given axis.
fn triangle_overlaps_aabb_on_axis(
    triangle: &Triangle,
    aabb: &Aabb,
    axis: &Unit<Vector3<f64>>,
) -> bool {
    let (min_tri, max_tri) = triangle.project_onto_axis(axis);
    let (min_aabb, max_aabb) = aabb.project_onto_axis(axis);

    !(max_tri < min_aabb || min_tri > max_aabb)
}
