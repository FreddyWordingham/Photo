struct Settings {
    width: u32,
    height: u32,
};

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

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<uniform> camera: Camera;

@group(0)
@binding(2)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;

    let pixel = vec2<i32>(i32(n), i32(m));

    let dx = ((f32(n) / f32(settings.width)) - 0.5) * 2.0;
    let dy = (((f32(m) / f32(settings.height)) - 0.5) * 2.0) / camera.aspect_ratio;
    let dr = sqrt(dx * dx + dy * dy);

    var colour = vec4<f32>(0.9, 0.9, 0.9, 1.0);
    if dr > 0.4 {
        colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }

    textureStore(texture, pixel, colour);
}
