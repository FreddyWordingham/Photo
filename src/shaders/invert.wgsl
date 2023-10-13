struct Settings {
    x: f32,
    y: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var read_texture: texture_storage_2d<rgba32float, read>;

@group(0)
@binding(2)
var write_texture: texture_storage_2d<rgba32float, write>;

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    let loc = vec2<u32>(x, y);
    let loc2 = vec2<u32>(x + u32(settings.x), y + u32(settings.y));

    var col = textureLoad(read_texture, loc);
    col.r = 1.0 - col.r;
    col.g = 1.0 - col.g;
    col.b = 1.0 - col.b;

    textureStore(write_texture, loc2, col);
}
