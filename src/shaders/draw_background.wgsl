struct Settings {
    width: u32,
    height: u32,
};

struct Camera {
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

    let dx = f32(n) - f32(settings.width) / 2.0;
    let dy = f32(m) - f32(settings.height) / 2.0;
    let r = sqrt(dx * dx + dy * dy) / f32(settings.width) * 2.0 * camera.zoom;

    var colour = vec4<f32>(0.0, 1.0, 1.0, 1.0);
    if r > 0.4 {
        colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }

    textureStore(texture, pixel, colour);
}
