@group(0)
@binding(0)
var<storage, read_write> chunks: array<Chunk>;
struct Chunk {
    col: vec4<f32>,
    x: f32,
    pad_a: f32,
    pad_b: f32,
    pad_c: f32
};

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var chunk = chunks[global_id.x];
    chunk.col.r = min(1.0, chunk.col.r + chunk.x);
    chunk.col.g = min(1.0, chunk.col.g + chunk.x);
    chunk.col.b = min(1.0, chunk.col.b + chunk.x);
    chunk.col.a = clamp(chunk.col.a, 0.0, 1.0);
    chunks[global_id.x] = chunk;
}
