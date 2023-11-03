@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x + (64u * settings.offset_x);
    let m = global_id.y + (64u * settings.offset_y);
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

    let state = trace(ray_pos, ray_dir);
    if (state.hit) {
        textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y - 1), vec4<f32>(state.colour, 1.0));
    }
}

struct RenderState {
    t: f32,
    colour: vec3<f32>,
    hit: bool,
}

fn trace(ray_pos: vec3<f32>, ray_dir: vec3<f32>) -> RenderState {
    var colour = vec3<f32>(0.2, 0.2, 0.2);

    var nearest_hit: f32 = 9999.0;
    var hit_something: bool = false;

    var render_state: RenderState;

    var node: BVHNode = bvh_data[0];
    var stack: array<BVHNode, 15>;
    var stack_index: i32 = 0;

    while (true) {
        var count: u32 = u32(node.count);
        var contents: u32 = u32(node.left_child);

        if (count == 0u) {
            var child_1: BVHNode = bvh_data[contents];
            var child_2: BVHNode = bvh_data[contents + 1u];

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
    return RenderState(nearest_hit, colour, hit_something);
}


fn hit_aabb(ray_pos: vec3<f32>, ray_dir: vec3<f32>, node: BVHNode) -> f32 {
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
