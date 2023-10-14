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
var<storage, read> ghost_forces: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> ghost_velocities: array<vec4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let dt = 0.0001;

    ghost_velocities[n].x += ghost_forces[n].x * dt;
    ghost_velocities[n].y += ghost_forces[n].y * dt;
    ghost_velocities[n].z += ghost_forces[n].z * dt;
}
