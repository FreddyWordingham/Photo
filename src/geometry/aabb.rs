use nalgebra::{Point3, Vector3};

pub struct Aabb {
    /// Minimum coordinates.
    pub mins: Point3<f64>,
    /// Maximum coordinates.
    pub maxs: Point3<f64>,
}

impl Aabb {
    /// Construct a new instance.
    pub fn new(mins: Point3<f64>, maxs: Point3<f64>) -> Self {
        Self { mins, maxs }
    }

    pub fn project_onto_axis(&self, axis: &Vector3<f64>) -> (f64, f64) {
        let corners = self.get_corners();
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;

        for corner in corners {
            let projection = &corner.coords.dot(axis);
            min = min.min(*projection);
            max = max.max(*projection);
        }

        (min, max)
    }

    fn get_corners(&self) -> Vec<Point3<f64>> {
        let mut corners = Vec::with_capacity(8);

        // Iterate over all combinations of mins and maxs for x, y, and z
        for &x in &[self.mins.x, self.maxs.x] {
            for &y in &[self.mins.y, self.maxs.y] {
                for &z in &[self.mins.z, self.maxs.z] {
                    corners.push(Point3::new(x, y, z));
                }
            }
        }

        corners
    }
}
