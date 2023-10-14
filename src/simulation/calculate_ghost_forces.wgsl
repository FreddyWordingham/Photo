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
var<storage, read> ghost_positions: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read> massive_positions: array<vec4<f32>>;

@group(0)
@binding(3)
var<storage, read_write> ghost_forces: array<vec4<f32>>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let num_massive_bodies = arrayLength(&massive_positions);

    let ghost_position = ghost_positions[n];
    var total_force = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    for (var i = 0u; i < num_massive_bodies; i = i + 1u) {
        let massive_position = massive_positions[i];
        let dx = massive_position.x - ghost_position.x;
        let dy = massive_position.y - ghost_position.y;
        let dz = massive_position.z - ghost_position.z;
        let r2 = dx * dx + dy * dy + dz * dz;
        let r = sqrt(r2);
        let f = settings.grav_force * massive_position.w / r2;

        total_force.x = total_force.x + f * dx / r;
        total_force.y = total_force.y + f * dy / r;
        total_force.z = total_force.z + f * dz / r;
        total_force.z = total_force.z + f;
    }

    ghost_forces[n] = total_force;
}
