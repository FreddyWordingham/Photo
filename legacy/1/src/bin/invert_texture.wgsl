struct Settings {
    x: f32,
    y: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var t: texture_storage_2d<rgba32float, read_write>;

const radius: i32 = 7;
const kernel: array<f32, 21> = array<f32, 21>(
    0.004,
    0.018,
    0.039,
    0.042,
    0.039,
    0.018,
    0.004, // Coefficients for radius 7
    0.018,
    0.081,
    0.178,
    0.191,
    0.178,
    0.081,
    0.018,
    0.039,
    0.178,
    0.389,
    0.417,
    0.389,
    0.178,
    0.039
);

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    if x == y {
        textureStore(t, vec2<u32>(x, y), vec4<f32>(0.0, 0.0, 1.0, 1.0));
        return;
    }

    var loc = vec2<u32>(x, y);

    var sum: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var dx: i32 = -radius; dx < radius; dx = dx + 1) {
        for (var dy: i32 = -radius; dy < radius; dy = dy + 1) {
            let offset_loc = vec2<i32>(i32(x) + dx, i32(y) + dy);
            // let weight = kernel[u32(radius + dx) + u32(radius + dy) * u32(2 * radius + 1)];
            let weight = 0.01;

            sum += weight * textureLoad(t, offset_loc);
        }
    }

    var col = textureLoad(t, loc);
    textureStore(t, loc, sum);
}
