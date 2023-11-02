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

    let normal = intersect_mesh_smooth_normal_and_distance(ray_pos, ray_dir);
    if normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0 {
        let new_colour = vec4<f32>(abs(normal.x), abs(normal.y), abs(normal.z), 1.0);
        textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y), new_colour);
    }
}

fn intersect_mesh_smooth_normal_and_distance(pos: vec3<f32>, dir: vec3<f32>) -> vec4<f32> {

    let num_triangles = arrayLength(&position_indices);
    var nearest = vec4<f32>(0.0, 0.0, 0.0, 1.0e9);

    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let p0 = positions[position_indices[n].x].xyz;
        let p1 = positions[position_indices[n].y].xyz;
        let p2 = positions[position_indices[n].z].xyz;

        let v0 = normals[normal_indices[n].x].xyz;
        let v1 = normals[normal_indices[n].y].xyz;
        let v2 = normals[normal_indices[n].z].xyz;

        let normal = intersect_triangle_smooth_normal_and_distance(pos, dir, p0, p1, p2, v0, v1, v2);

        if (normal.w < nearest.w) && (normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0) {
            nearest = normal;
        }
    }

    return nearest;
}

fn intersect_triangle_smooth_normal_and_distance(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, n0: vec3<f32>, n1: vec3<f32>, n2: vec3<f32>) -> vec4<f32> {
    let e1 = p1 - p0;
    let e2 = p2 - p0;

    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        let n = n0 * (1.0 - u - v) + n1 * u + n2 * v;
        return vec4<f32>(normalize(n), t);
    }

    return vec4<f32>(0.0, 0.0, 0.0, -1.0);
}


//

fn intersect_mesh_normal_and_distance(pos: vec3<f32>, dir: vec3<f32>) -> vec4<f32> {

    let num_triangles = arrayLength(&position_indices);
    var nearest = vec4<f32>(0.0, 0.0, 0.0, 1.0e9);

    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let p0 = positions[position_indices[n].x].xyz;
        let p1 = positions[position_indices[n].y].xyz;
        let p2 = positions[position_indices[n].z].xyz;

        let v0 = normals[normal_indices[n].x].xyz;
        let v1 = normals[normal_indices[n].y].xyz;
        let v2 = normals[normal_indices[n].z].xyz;

        let normal = intersect_triangle_normal_and_distance(pos, dir, p0, p1, p2);

        if (normal.w < nearest.w) && (normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0) {
            nearest = normal;
        }
    }

    return nearest;
}

fn intersect_triangle_normal_and_distance(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>) -> vec4<f32> {
    let e1 = p1 - p0;
    let e2 = p2 - p0;

    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return vec4<f32>(0.0, 0.0, 0.0, -1.0);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        return vec4<f32>(normalize(cross(e1, e2)), t);
    }

    return vec4<f32>(0.0, 0.0, 0.0, -1.0);
}
