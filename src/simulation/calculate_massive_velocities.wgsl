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
var<storage, read> massive_forces: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> massive_velocities: array<vec4<f32>>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let dt = 0.001;

    massive_velocities[n].x += massive_forces[n].x * dt / massive_velocities[n].w;
    massive_velocities[n].y += massive_forces[n].y * dt / massive_velocities[n].w;
    massive_velocities[n].z += massive_forces[n].z * dt / massive_velocities[n].w;
}
