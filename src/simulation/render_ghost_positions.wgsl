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
var texture: texture_storage_2d<rgba32float, read_write>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let pixel = position_to_pixel(positions[n].x, positions[n].y);
    let colour = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    let prev_colour = textureLoad(texture, pixel);
    let new_colour = prev_colour + colour;

    textureStore(texture, pixel, new_colour);
}

fn position_to_pixel(x: f32, y: f32) -> vec2<i32> {
    let col = (x * settings.zoom) + (settings.nrows / 2.0);
    let row = (y * settings.zoom) + (settings.ncols / 2.0);
    return vec2<i32>(i32(col), i32(row));
}
