use nalgebra::{Point3, Similarity3, Unit, Vector3};

use crate::{
    geometry::{Aabb, Ray},
    world::Mesh,
};

/// Instance of a renderable object.
pub struct Object {
    /// Mesh id.
    mesh_id: String,
    /// Object to world coordinate transformation.
    transformation: Similarity3<f64>,
    /// World to object coordinate transformation.
    inverse_transformation: Similarity3<f64>,
    /// Axis-aligned bounding box.
    aabb: Aabb,
}

impl Object {
    /// Construct a new instance.
    pub fn new(mesh_id: String, mesh: &Mesh, transformation: Similarity3<f64>) -> Self {
        debug_assert!(!mesh_id.is_empty());

        let mut mins = Point3::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
        let mut maxs = Point3::new(std::f64::MIN, std::f64::MIN, std::f64::MIN);

        // Transform each vertex position and update the bounding box.
        for vertex_position in mesh.vertex_positions() {
            let transformed_vertex_position = transformation * vertex_position;
            mins.x = mins.x.min(transformed_vertex_position.x);
            mins.y = mins.y.min(transformed_vertex_position.y);
            mins.z = mins.z.min(transformed_vertex_position.z);
            maxs.x = maxs.x.max(transformed_vertex_position.x);
            maxs.y = maxs.y.max(transformed_vertex_position.y);
            maxs.z = maxs.z.max(transformed_vertex_position.z);
        }

        Self {
            mesh_id,
            transformation,
            inverse_transformation: transformation.inverse(),
            aabb: Aabb::new(mins, maxs),
        }
    }

    /// Get the mesh id.
    pub fn mesh_id(&self) -> &str {
        &self.mesh_id
    }

    /// Get the axis-aligned bounding box.
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }

    /// Ray intersection test.
    pub fn intersect_ray(&self, ray: &Ray, mesh: &Mesh) -> bool {
        if !self.aabb.intersect_ray(ray) {
            return false;
        }

        let transformed_ray = Ray::new(
            self.inverse_transformation * ray.origin,
            Unit::new_normalize(self.inverse_transformation * ray.direction.as_ref()),
        );

        mesh.intersect_ray(&transformed_ray)
    }

    /// Ray intersection distance and normal.
    pub fn intersect_ray_distance_normal(
        &self,
        ray: &Ray,
        mesh: &Mesh,
    ) -> Option<(f64, Unit<Vector3<f64>>)> {
        if !self.aabb.intersect_ray(ray) {
            return None;
        }

        let transformed_ray = Ray::new(
            self.inverse_transformation * ray.origin,
            Unit::new_normalize(self.inverse_transformation * ray.direction.as_ref()),
        );

        if let Some((distance, normal)) = mesh.intersect_ray_distance_normal(&transformed_ray) {
            return Some((
                distance,
                Unit::new_normalize(self.transformation * normal.as_ref()),
            ));
        }

        None
    }
}
