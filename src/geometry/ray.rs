use nalgebra::{Point3, Unit, Vector3};

pub struct Ray {
    /// Origin position.
    pub origin: Point3<f64>,
    /// Origin direction.
    pub direction: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    pub fn new(origin: Point3<f64>, direction: Unit<Vector3<f64>>) -> Self {
        Self { origin, direction }
    }

    /// Travel (move the origin) along the ray's direction.
    pub fn travel(&mut self, distance: f64) {
        self.origin += distance * self.direction.as_ref();
    }
}
