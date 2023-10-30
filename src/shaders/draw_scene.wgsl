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
var<storage, read> triangles: array<vec3<f32>>;

@group(0)
@binding(4)
var<storage, read> indices: array<vec3<u32>>;

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;

    let px = ((f32(n) / f32(settings.width)) - 0.5) * 2.0;
    let py = -1.0;
    let pz = ((f32(m) / f32(settings.height)) - 0.5) * 2.0;

    let pos = vec3<f32>(px, py, pz);
    let dir = vec3<f32>(0.0, 1.0, 0.0);

    if intersect_mesh(pos, dir) {
        let pixel = vec2<i32>(i32(n), i32(m));
        let old_colour = textureLoad(texture, pixel);
        let new_colour = vec4<f32>(1.0, 0.0, 0.0, 0.5);
        textureStore(texture, pixel, old_colour * 0.5 + new_colour * 0.5);
    }
}

fn intersect_mesh(pos: vec3<f32>, dir: vec3<f32>) -> bool {

    let num_triangles = arrayLength(&indices);
    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let i = indices[n].x;
        let j = indices[n].y;
        let k = indices[n].z;

        let v0 = triangles[i];
        let v1 = triangles[j];
        let v2 = triangles[k];

        if intersect_triangle(pos, dir, v0, v1, v2) {
            return true;
        }
    }

    return false;
}

fn intersect_triangle(pos: vec3<f32>, dir: vec3<f32>, v0: vec3<f32>, v1: vec3<f32>, v2: vec3<f32>) -> bool {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let h = cross(dir, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return false;
    }

    let f = 1.0 / a;
    let s = pos - v0;
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
