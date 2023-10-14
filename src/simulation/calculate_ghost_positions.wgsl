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
var<storage, read> ghost_velocities: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> ghost_positions: array<vec4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let dt = 0.001;

    ghost_positions[n].x += ghost_velocities[n].x * dt;
    ghost_positions[n].y += ghost_velocities[n].y * dt;
    ghost_positions[n].z += ghost_velocities[n].z * dt;
}
