struct Sample {
    hit: bool,
    distance: f32,
    index: u32,
}

@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;
    let pixel = vec2<i32>(i32(n), i32(m));

    let ray = create_ray(pixel);
    let sample = intersect_mesh(ray);

    if sample.hit {
        let hue = f32(sample.index) / 10.0;
        let hsva = vec4<f32>(hue, 0.5, 0.5, 1.0);
        let new_colour = hsva_to_rgba(hsva);
        textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y - 1), new_colour);
    }
}

fn intersect_mesh(ray: Ray) -> Sample {

    let num_triangles = arrayLength(&position_indices);

    var sample = Sample(false, 0.0, 0u);

    for (var n = 0u; n < num_triangles; n = n + 1u) {
        let indices = position_indices[n];

        let p0 = positions[indices.x].xyz;
        let p1 = positions[indices.y].xyz;
        let p2 = positions[indices.z].xyz;

        let index = u32(positions[indices.x].w);

        let new_sample = intersect_triangle(ray, p0, p1, p2, index);

        if new_sample.hit {
            if !sample.hit || new_sample.distance < sample.distance {
                sample = new_sample;
            }
        }
    }

    return sample;
}

fn intersect_triangle(ray: Ray, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, index: u32) -> Sample {
    let e2 = p2 - p0;
    let e1 = p1 - p0;
    let h = cross(ray.direction, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        let s = Sample(false, 0.0, 0u);
        return s;
    }

    let f = 1.0 / a;
    let s = ray.origin - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return Sample(false, 0.0, 0u);
    }

    let q = cross(s, e1);
    let v = f * dot(ray.direction, q);

    if v < 0.0 || u + v > 1.0 {
        return Sample(false, 0.0, 0u);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        return Sample(true, t, index);
    }

    return Sample(false, 0.0, 0u);
}
