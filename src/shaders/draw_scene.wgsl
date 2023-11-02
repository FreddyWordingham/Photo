struct Settings {
    width: u32,
    height: u32,
    _padding: vec2<u32>,
};

struct Camera {
    eye_position_x: f32,
    eye_position_y: f32,
    eye_position_z: f32,
    target_position_x: f32,
    target_position_y: f32,
    target_position_z: f32,
    up_direction_x: f32,
    up_direction_y: f32,
    up_direction_z: f32,
    aspect_ratio: f32,
    fov_y: f32,
    zoom: f32,
};

struct Node {
    mins: vec3<f32>,
    left_child: f32,
    maxs: vec3<f32>,
    count: f32,
}

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<uniform> camera: Camera;

@group(0)
@binding(2)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@group(0)
@binding(3)
var<storage, read> positions: array<vec4<f32>>;

@group(0)
@binding(4)
var<storage, read> normals: array<vec4<f32>>;

@group(0)
@binding(5)
var<storage, read> position_indices: array<vec3<u32>>;

@group(0)
@binding(6)
var<storage, read> normal_indices: array<vec3<u32>>;

@group(0)
@binding(7)
var<storage, read> bvh_data: array<Node>;

@group(0)
@binding(8)
var<storage, read> bvh_indices: array<u32>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;
    let pixel = vec2<i32>(i32(n), i32(m));

    let eye_pos = vec3<f32>(camera.eye_position_x, camera.eye_position_y, camera.eye_position_z);
    let target_pos = vec3<f32>(camera.target_position_x, camera.target_position_y, camera.target_position_z);
    let forward_dir = normalize(target_pos - eye_pos);
    let right_dir = normalize(cross(forward_dir, vec3<f32>(camera.up_direction_x, camera.up_direction_y, camera.up_direction_z)));
    let up_dir = normalize(cross(right_dir, forward_dir));
    let fov_x = camera.aspect_ratio * camera.fov_y;

    let ray_pos = eye_pos;

    let ray_dir = normalize(
        forward_dir * camera.zoom + right_dir * (2.0 * (f32(pixel.x) / f32(settings.width)) - 1.0) * tan(fov_x / 2.0) + up_dir * (2.0 * (f32(pixel.y) / f32(settings.height)) - 1.0) * tan(camera.fov_y / 2.0)
    );

    if intersect_mesh(ray_pos, ray_dir) {
        let new_colour = vec4<f32>(0.3, 0.1, 0.3, 1.0);
        textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y + 1), new_colour);
    }
}

fn intersect_mesh(pos: vec3<f32>, dir: vec3<f32>) -> bool {

    let num_triangles = arrayLength(&position_indices);
    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let p0 = positions[position_indices[n].x].xyz;
        let p1 = positions[position_indices[n].y].xyz;
        let p2 = positions[position_indices[n].z].xyz;

        if intersect_triangle(pos, dir, p0, p1, p2) {
            return true;
        }
    }

    return false;
}

fn intersect_triangle(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>) -> bool {
    let e2 = p2 - p0;
    let e1 = p1 - p0;
    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return false;
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return false;
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        return true;
    }

    return false;
}
