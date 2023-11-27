use nalgebra::{Unit, Vector3};

use crate::assets::Material;

pub struct Hit<'a> {
    pub is_inside: bool,
    pub distance: f64,
    pub normal: Unit<Vector3<f64>>,
    pub smooth_normal: Unit<Vector3<f64>>,
    pub material: &'a Material,
}

impl<'a> Hit<'a> {
    pub fn new(
        is_inside: bool,
        distance: f64,
        normal: Unit<Vector3<f64>>,
        smooth_normal: Unit<Vector3<f64>>,
        material: &'a Material,
    ) -> Self {
        Self {
            is_inside,
            distance,
            normal,
            smooth_normal,
            material,
        }
    }
}
