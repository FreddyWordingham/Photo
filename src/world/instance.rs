use nalgebra::{Point3, Similarity3};

use crate::{
    assets::{Material, Mesh},
    geometry::{Aabb, Ray},
};

pub struct Instance<'a> {
    mesh: &'a Mesh,
    _material: &'a Material,
    transformation: Similarity3<f64>,
    inverse_transformation: Similarity3<f64>,
    aabb: Aabb,
}

impl<'a> Instance<'a> {
    pub fn new(mesh: &'a Mesh, material: &'a Material, transformation: Similarity3<f64>) -> Self {
        let inverse_transformation = transformation.inverse();

        let aabb = Self::init_aabb(mesh, &transformation);

        Self {
            mesh,
            _material: material,
            transformation,
            inverse_transformation,
            aabb,
        }
    }

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

    pub fn aabb(&self) -> Aabb {
        self.aabb
    }

    pub fn ray_intersect(&self, ray: &Ray) -> bool {
        let transformed_ray = ray * &self.inverse_transformation;
        self.mesh.ray_intersect(&transformed_ray)
    }

    pub fn ray_intersect_distance(&self, ray: &Ray) -> Option<f64> {
        let transformed_ray = ray * &self.inverse_transformation;
        if let Some(distance) = self.mesh.ray_intersect_distance(&transformed_ray) {
            Some(distance * self.transformation.scaling())
        } else {
            None
        }
    }
}
