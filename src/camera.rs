use cgmath::prelude::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct Camera {
    position: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect_ratio: f32,
    vertical_field_of_view: f32,
    near_clip: f32,
    far_clip: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.position, self.target, self.up);
        let proj = cgmath::perspective(
            cgmath::Deg(self.vertical_field_of_view),
            self.aspect_ratio,
            self.near_clip,
            self.far_clip,
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
