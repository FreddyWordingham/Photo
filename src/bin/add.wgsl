struct Chunk {
    col: vec4<f32>,
};

struct Settings {
    x: f32,
    y: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<storage, read_write> chunks: array<Chunk>;

@compute
@workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var chunk = chunks[global_id.x];
    var p = settings.x;
    chunk.col.r = 1.0 - chunk.col.r;
    chunk.col.g = 1.0 - chunk.col.g;
    chunk.col.b = 1.0 - chunk.col.b;
    chunk.col.a = chunk.col.a;
    chunks[global_id.x] = chunk;
}
