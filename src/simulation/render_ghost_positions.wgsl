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
var<storage, read> positions: array<vec4<f32>>;

@group(0)
@binding(2)
var texture: texture_storage_2d<rgba32float, write>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let pixel = position_to_pixel(positions[n].x, positions[n].y);
    let colour = kind_to_colour(positions[n].w);

    textureStore(texture, pixel, colour);
}

fn position_to_pixel(x: f32, y: f32) -> vec2<i32> {
    let col = (x * settings.zoom) + (settings.ncols / 2.0);
    let row = (y * settings.zoom) + (settings.nrows / 2.0);
    return vec2<i32>(i32(row), i32(col));
}

fn kind_to_colour(kind: f32) -> vec4<f32> {
    return scalar_to_rainbow(kind / 5.0);
}

fn scalar_to_rainbow(x: f32) -> vec4<f32> {
    if x < 0.25 {
        return vec4<f32>(0.0, 4.0 * x, 1.0, 1.0);
    } else if x < 0.5 {
        return vec4<f32>(0.0, 1.0, 1.0 - 4.0 * (x - 0.25), 1.0);
    } else if x < 0.75 {
        return vec4<f32>(4.0 * (x - 0.5), 1.0, 0.0, 1.0);
    }
    return vec4<f32>(1.0, 1.0 - 4.0 * (x - 0.75), 0.0, 1.0);
}
