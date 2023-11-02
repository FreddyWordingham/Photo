struct Camera {
    eye_position_x: f32,
    eye_position_y: f32,
    eye_position_z: f32,
    target_position_x: f32,
    target_position_y: f32,
    target_position_z: f32,
    up_direction_x: f32,
    up_direction_y: f32,
    up_direction_z: f32,
    aspect_ratio: f32,
    fov_y: f32,
    zoom: f32,
};

fn create_ray(pixel: vec2<i32>) -> Ray {
    let eye_pos = vec3<f32>(camera.eye_position_x, camera.eye_position_y, camera.eye_position_z);
    let target_pos = vec3<f32>(camera.target_position_x, camera.target_position_y, camera.target_position_z);
    let forward_dir = normalize(target_pos - eye_pos);
    let right_dir = normalize(cross(forward_dir, vec3<f32>(camera.up_direction_x, camera.up_direction_y, camera.up_direction_z)));
    let up_dir = normalize(cross(right_dir, forward_dir));
    let fov_x = camera.aspect_ratio * camera.fov_y;

    return Ray(eye_pos, normalize(forward_dir * camera.zoom + right_dir * (2.0 * (f32(pixel.x) / f32(settings.width)) - 1.0) * tan(fov_x / 2.0) + up_dir * (2.0 * (f32(pixel.y) / f32(settings.height)) - 1.0) * tan(camera.fov_y / 2.0)));
}
