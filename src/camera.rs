use nalgebra::{Rotation3, Unit, Vector3};

pub struct Camera {
    pub eye_position: Vector3<f32>,
    pub target_position: Vector3<f32>,
    pub upward_direction: Unit<Vector3<f32>>,
    pub aspect_ratio: f32,
    pub fov_y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(
        eye_position: [f32; 3],
        target_position: [f32; 3],
        upward_direction: [f32; 3],
        aspect_ratio: f32,
        fov_x: f32,
        zoom: f32,
    ) -> Self {
        debug_assert!(eye_position
            .iter()
            .zip(target_position.iter())
            .any(|(&a, &b)| a != b));
        debug_assert!(upward_direction.iter().fold(0.0, |acc, &x| acc + x.abs()) > 0.0);
        debug_assert!(aspect_ratio > 0.0);
        debug_assert!(fov_x > 0.0);
        debug_assert!(zoom > 0.0);

        let fov_y = fov_x * aspect_ratio;

        Self {
            eye_position: eye_position.into(),
            target_position: target_position.into(),
            upward_direction: Unit::new_normalize(upward_direction.into()),
            aspect_ratio,
            fov_y,
            zoom,
        }
    }

    pub fn as_buffer(&self) -> Vec<f32> {
        vec![
            self.eye_position.x,
            self.eye_position.y,
            self.eye_position.z,
            self.target_position.x,
            self.target_position.y,
            self.target_position.z,
            self.upward_direction.x,
            self.upward_direction.y,
            self.upward_direction.z,
            self.aspect_ratio,
            self.fov_y,
            self.zoom,
        ]
    }

    pub fn is_valid(&self) -> bool {
         self.aspect_ratio > 0.0
            && self.fov_y > 0.0
            && self.zoom > 0.0
    }

    // Rotate the camera about the up axis, centred on the target position
    pub fn rotate_azimuthal(&mut self, delta: f32) {
        let rotation = Rotation3::from_axis_angle(&self.upward_direction, delta);
        let displacement = self.eye_position - self.target_position;
        self.eye_position = rotation * displacement + self.target_position;        
    }

    pub fn rotate_polar(&mut self, delta: f32) {
        let forward = Unit::new_normalize(self.target_position - self.eye_position);
        let right = Unit::new_normalize(forward.cross(&self.upward_direction));
        let rotation = Rotation3::from_axis_angle(&right, -delta);

        let displacement = self.eye_position - self.target_position;
        self.eye_position = rotation * displacement + self.target_position;
        self.upward_direction = rotation * self.upward_direction;
    }

    pub fn magnify(&mut self, delta: f32) {
        self.zoom *= delta;
    }
}
