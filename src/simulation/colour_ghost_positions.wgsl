struct Settings {
    nrows: f32,
    ncols: f32,
    grav_force: f32,
    zoom: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var texture: texture_storage_2d<rgba32float, read_write>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    let pixel = vec2<u32>(x, y);

    let prev_colour = textureLoad(texture, pixel);
    let total: f32 = min(1.0, prev_colour.r / 50.0);
    textureStore(texture, pixel, vec4<f32>(0.0, 0.0, total, 1.0));

    // if total <= 0.0 {
    //     textureStore(texture, pixel, vec4<f32>(0.0, 0.0, 0.0, 1.0));
    //     return;
    // }

    // let r = sin(3.14159 * (total + 1.0)) * 0.5 + 0.5;
    // let g = sin(3.14159 * (total + 0.5)) * 0.5 + 0.5;
    // let b = sin(3.14159 * total) * 0.5 + 0.5;
    // let new_colour = vec4<f32>(r, g, b, 1.0);

    // textureStore(texture, pixel, new_colour);
}
