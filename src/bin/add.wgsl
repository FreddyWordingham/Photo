struct Chunk {
    col: vec4<f32>,
    x: f32
};

struct Settings {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    v: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<storage, read_write> chunks: array<Chunk>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var chunk = chunks[global_id.x];
    var p = settings.x;
    chunk.col.r = chunk.x;
    chunk.col.g = chunk.x;
    chunk.col.b = chunk.x;
    chunk.col.a = chunk.x;
    chunks[global_id.x] = chunk;
}
