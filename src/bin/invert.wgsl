struct Chunk {
    col: vec4<f32>
};

struct World {
    player_x: f32,
    player_y: f32,
};

@group(0)
@binding(0)
var<uniform> world: World;

@group(0)
@binding(1)
var<storage, read_write> chunks: array<Chunk>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var chunk = chunks[global_id.x];
    var px = world.player_x;
    var py = world.player_y;
    chunk.col.r = chunk.col.r;
    chunk.col.g = chunk.col.g;
    chunk.col.b = chunk.col.b;
    chunk.col.a = chunk.col.a;
    chunks[global_id.x] = chunk;
}
