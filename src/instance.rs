use nalgebra::Similarity3;

use crate::geometry::{Bvh, Mesh};

pub struct Instance<'a> {
    _mesh: &'a Mesh,
    _transformation: Similarity3<f64>,
    _inverse_transformation: Similarity3<f64>,
    _bvh: Bvh,
}

impl<'a> Instance<'a> {
    pub fn new(mesh: &'a Mesh, transformation: Similarity3<f64>) -> Self {
        let inverse_transformation = transformation.inverse();
        Self {
            _mesh: mesh,
            _transformation: transformation,
            _inverse_transformation: inverse_transformation,
            _bvh: Bvh {},
        }
    }
}
