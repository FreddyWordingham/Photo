use crate::geometry::Scene;

pub struct BVH {
    pub tree: Vec<f32>,
}

impl BVH {
    pub fn new(_scene: &Scene) -> Self {
        let tree = vec![];

        Self { tree }
    }
}
