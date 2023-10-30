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
var<storage, read> massive_positions: array<vec4<f32>>;

@group(0)
@binding(2)
var<storage, read_write> massive_forces: array<vec4<f32>>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;

    let num_massive_bodies = arrayLength(&massive_positions);

    var total_force = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    for (var i = 0u; i < num_massive_bodies; i = i + 1u) {
        if i == n {
            continue;
        }

        let pos = massive_positions[i].xyz;
        let dx = pos.x - massive_positions[n].x;
        let dy = pos.y - massive_positions[n].y;
        let dz = pos.z - massive_positions[n].z;
        let r2 = (dx * dx + dy * dy + dz * dz) + 0.01;
        let r = sqrt(r2);
        let f = settings.grav_force * massive_positions[n].w * massive_positions[i].w / r2;

        total_force.x = total_force.x + f * dx / r;
        total_force.y = total_force.y + f * dy / r;
        total_force.z = total_force.z + f * dz / r;
    }

    massive_forces[n] = total_force;
}
