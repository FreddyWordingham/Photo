struct Settings {
    num_bodies: f32,
    grav_force: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<storage, read> positions: array<vec2<f32>>;

@group(0)
@binding(2)
var<storage, read> velocities: array<vec2<f32>>;

@group(0)
@binding(3)
var<storage, read> masses: array<f32>;

@group(0)
@binding(4)
var<storage, read_write> forces: array<vec2<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let pos = positions[n];
    let vel = velocities[n];
    let mass = masses[n];

    var force = vec2<f32>(0.0, 0.0);

    // for (var i = 0u; i < u32(settings.num_bodies); i = i + 1u) {
    for (var i = 0u; i < u32(2); i = i + 1u) {
        if i == n {
            continue;
        }

        let other_pos = positions[i];
        let other_mass = masses[i];

        let diff = other_pos - pos;
        let dist = length(diff);
        let dir = diff / dist;

        let f = dir * (mass * other_mass) / (dist * dist);

        force = force + f;
    }

    // if force[0] >= 1.0 {
    //     force[0] = 1.0;
    // }
    // if force[1] >= 1.0 {
    //     force[1] = 1.0;
    // }

    forces[n] = force * settings.grav_force;
}
