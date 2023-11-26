use nalgebra::{Point3, Similarity3};

use crate::{
    assets::{Material, Mesh},
    geometry::{Aabb, Ray},
    // world::Bvh,
};

pub struct Instance<'a> {
    mesh: &'a Mesh,
    _material: &'a Material,
    _transformation: Similarity3<f64>,
    inverse_transformation: Similarity3<f64>,
    // _bvh: Bvh,
    aabb: Aabb,
}

impl<'a> Instance<'a> {
    pub fn new(mesh: &'a Mesh, material: &'a Material, transformation: Similarity3<f64>) -> Self {
        let inverse_transformation = transformation.inverse();

        let aabb = Self::init_aabb(mesh, &transformation);

        Self {
            mesh,
            _material: material,
            _transformation: transformation,
            inverse_transformation,
            // _bvh: Bvh::new(&vec![]),
            aabb,
        }
    }

    fn init_aabb(mesh: &Mesh, transformation: &Similarity3<f64>) -> Aabb {
        let (mut mins, mut maxs) = (
            Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        );

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

    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        let transformed_ray = ray * &self.inverse_transformation;
        self.mesh
            .triangles()
            .any(|triangle| triangle.intersect_ray(&transformed_ray))
    }
}
