struct Settings {
    width: u32,
    height: u32,
};

struct Camera {
    zoom: f32,
};

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
var<storage, read> positions: array<vec3<f32>>;

@group(0)
@binding(4)
var<storage, read> normals: array<vec3<f32>>;

@group(0)
@binding(5)
var<storage, read> position_indices: array<vec3<u32>>;

@group(0)
@binding(6)
var<storage, read> normal_indices: array<vec3<u32>>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;

    let px = ((f32(n) / f32(settings.width)) - 0.5) / camera.zoom;
    let py = -1.0;
    let pz = ((f32(m) / f32(settings.height)) - 0.5) / camera.zoom;

    let pos = vec3<f32>(px, py, pz);
    let dir = vec3<f32>(0.0, 1.0, 0.0);

    let normal = intersect_mesh_normal(pos, dir);

    if normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0 {
        let pixel = vec2<i32>(i32(n), i32(m));
        let new_colour = vec4<f32>(abs(normal), 0.5);
        textureStore(texture, pixel, new_colour);
    }
}

fn intersect_mesh_normal(pos: vec3<f32>, dir: vec3<f32>) -> vec3<f32> {

    let num_triangles = arrayLength(&position_indices);
    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let p0 = positions[position_indices[n].x];
        let p1 = positions[position_indices[n].y];
        let p2 = positions[position_indices[n].z];

        let v0 = normals[normal_indices[n].x];
        let v1 = normals[normal_indices[n].y];
        let v2 = normals[normal_indices[n].z];

        // let normal = intersect_triangle_normal(pos, dir, p0, p1, p2);
        let normal = intersect_triangle_smooth_normal(pos, dir, p0, p1, p2, v0, v1, v2);

        if normal.x != 0.0 || normal.y != 0.0 || normal.z != 0.0 {
            return normal;
        }
    }

    return vec3<f32>(0.0, 0.0, 0.0);
}

fn intersect_triangle_normal(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>) -> vec3<f32> {
    let e1 = p1 - p0;
    let e2 = p2 - p0;

    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        return normalize(cross(e1, e2));
    }

    return vec3<f32>(0.0, 0.0, 0.0);
}

fn intersect_triangle_smooth_normal(pos: vec3<f32>, dir: vec3<f32>, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, n0: vec3<f32>, n1: vec3<f32>, n2: vec3<f32>) -> vec3<f32> {
    let e1 = p1 - p0;
    let e2 = p2 - p0;

    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let f = 1.0 / a;
    let s = pos - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let q = cross(s, e1);
    let v = f * dot(dir, q);

    if v < 0.0 || u + v > 1.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        let n = n0 * (1.0 - u - v) + n1 * u + n2 * v;
        return normalize(n);
    }

    return vec3<f32>(0.0, 0.0, 0.0);
}