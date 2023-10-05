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
    chunk.col.r = clamp(1.0 - chunk.col.r, 0.0, 1.0);
    chunk.col.g = clamp(1.0 - chunk.col.g, 0.0, 1.0);
    chunk.col.b = clamp(1.0 - chunk.col.b, 0.0, 1.0);
    chunk.col.a = clamp(chunk.col.a, 0.0, 1.0);
    chunks[global_id.x] = chunk;
}
