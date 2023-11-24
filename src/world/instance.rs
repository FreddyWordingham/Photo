use nalgebra::Similarity3;

use crate::{
    assets::{Material, Mesh},
    world::Bvh,
};

pub struct Instance<'a> {
    _mesh: &'a Mesh,
    _material: &'a Material,
    _transformation: Similarity3<f64>,
    _inverse_transformation: Similarity3<f64>,
    _bvh: Bvh,
}

impl<'a> Instance<'a> {
    pub fn _new(mesh: &'a Mesh, material: &'a Material, transformation: Similarity3<f64>) -> Self {
        let inverse_transformation = transformation.inverse();
        Self {
            _mesh: mesh,
            _material: material,
            _transformation: transformation,
            _inverse_transformation: inverse_transformation,
            _bvh: Bvh {},
        }
    }
}
