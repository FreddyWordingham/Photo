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

    // let normal = intersect_mesh_normal_and_distance(ray_pos, ray_dir);
    // if normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0 {
    //     let new_colour = vec4<f32>(abs(normal.x), abs(normal.y), abs(normal.z), 1.0);
    //     textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y + 1), new_colour);
    // }

    let colour = trace(ray_pos, ray_dir);
    textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y), vec4<f32>(colour, 1.0));
}


struct RenderState {
    t: f32,
    colour: vec3<f32>,
    hit: bool,
}

fn trace(ray_pos: vec3<f32>, ray_dir: vec3<f32>) -> vec3<f32> {
    var colour = vec3<f32>(0.2, 0.2, 0.2);

    var nearest_hit: f32 = 9999.0;
    var hit_something: bool = false;

    var render_state: RenderState;

    var node: Node = bvh_data[0];
    var stack: array<Node, 15>;
    var stack_index: i32 = 0;

    while (true) {
        var count: u32 = u32(node.count);
        var contents: u32 = u32(node.left_child);

        if (count == 0u) {
            var child_1: Node = bvh_data[contents];
            var child_2: Node = bvh_data[contents + 1u];

            var distance_1 = hit_aabb(ray_pos, ray_dir, child_1);
            var distance_2 = hit_aabb(ray_pos, ray_dir, child_2);

            if (distance_1 > distance_2) {
                var temp_distance = distance_1;
                distance_1 = distance_2;
                distance_2 = temp_distance;

                var temp_node = child_1;
                child_1 = child_2;
                child_2 = temp_node;
            }

            if (distance_1 > nearest_hit) {
                if (stack_index == 0) {
                    break;
                }
                else {
                    stack_index -= 1;
                    node = stack[stack_index];
                }
            }
            else {
                node = child_1;
                if (distance_2 < nearest_hit) {
                    stack[stack_index] = child_2;
                    stack_index += 1;
                }
            }
        }
        else {
            for (var i: u32 = 0u; i < count; i++) {
                let tri_index: vec3<u32> = position_indices[bvh_indices[contents + i]];

                var new_render_state: RenderState = hit_triangle(
                    ray_pos,
                    ray_dir,
                    vec3<f32>(positions[tri_index.x].xyz),
                    vec3<f32>(positions[tri_index.y].xyz),
                    vec3<f32>(positions[tri_index.z].xyz)
                );

                if (new_render_state.hit) {
                    hit_something = true;

                    if (new_render_state.t < nearest_hit) {
                        nearest_hit = new_render_state.t;
                        render_state = new_render_state;
                    }
                }
            }

            if (stack_index == 0) {
                break;
            }
            else {
                stack_index -= 1;
                node = stack[stack_index];
            }
        }
    }

    if (hit_something) {
        colour = render_state.colour;
    }
    return colour;
}


fn hit_aabb(ray_pos: vec3<f32>, ray_dir: vec3<f32>, node: Node) -> f32 {
    var inverse_dir: vec3<f32> = vec3(1.0) / ray_dir;
    var t1: vec3<f32> = (node.mins - ray_pos) * inverse_dir;
    var t2: vec3<f32> = (node.maxs - ray_pos) * inverse_dir;
    var tMin: vec3<f32> = min(t1, t2);
    var tMax: vec3<f32> = max(t1, t2);

    var t_min: f32 = max(max(tMin.x, tMin.y), tMin.z);
    var t_max: f32 = min(min(tMax.x, tMax.y), tMax.z);

    if (t_min > t_max || t_max < 0.0) {
        return 99999.0;
    }
    else {
        return t_min;
    }
}

fn hit_triangle(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>) -> RenderState {
    let e1 = p1 - p0;
    let e2 = p2 - p0;

    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return RenderState(0.0, vec3<f32>(0.0, 0.0, 0.0), false);
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return RenderState(0.0, vec3<f32>(0.0, 0.0, 0.0), false);
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return RenderState(0.0, vec3<f32>(0.0, 0.0, 0.0), false);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        let normal = normalize(cross(e1, e2));
        let colour = vec3<f32>(abs(normal.x), abs(normal.y), abs(normal.z));
        return RenderState(t, colour, true);
    }

    return RenderState(0.0, vec3<f32>(0.0, 0.0, 0.0), false);
}
