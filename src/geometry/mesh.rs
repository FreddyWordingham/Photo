use crate::geometry::{Aabb, Ray, Triangle};

struct Face {}

pub struct Mesh {
    aabb: Aabb,
    faces: Vec<Face>,
}

impl Mesh {
    /// Iterate over the triangles of the mesh.
    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.faces.iter().map(|_| Triangle {})
    }

    /// Test for an intersection with a ray.
    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        if !self.aabb.intersect_ray(ray) {
            return false;
        }

        for triangle in self.triangles() {
            if triangle.intersect_ray(ray) {
                return true;
            }
        }

        false
    }
}
