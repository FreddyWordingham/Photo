struct Sample {
    hit: bool,
    normal: vec3<f32>,
    distance: f32,
    index: u32,
}

const SUN_POSITION: vec3<f32> = vec3<f32>(1.0e3, 2.0e3, 10.0e3);

@compute
@workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let n = global_id.x;
    let m = global_id.y;
    let pixel = vec2<i32>(i32(n), i32(m));

    var ray = create_ray(pixel);

    for (var i: i32 = 0; i < 5; i++) {
        let sample = trace(ray);

        if sample.hit {
            if sample.index == 0u {
                let hit_pos = ray.origin + (ray.direction * (sample.distance - 0.0001));
                let reflected_dir = reflect(ray.direction, sample.normal);
                ray = Ray(hit_pos, reflected_dir);
                continue;
            }
            else 
            {
                let hit_pos = ray.origin + (ray.direction * (sample.distance - 0.0001));
                let sun_dir = normalize(SUN_POSITION - hit_pos);
                let light_ray = Ray(hit_pos, sun_dir);

                let light_sample = trace(light_ray);
                
                var lightness = dot(light_ray.direction, sample.normal);

                if light_sample.hit {
                    var colour = vec4<f32>(0.0, 0.0, 0.0, 1.0);
                    if light_sample.distance < 2.0 {
                        lightness *= (light_sample.distance / 2.0) * (light_sample.distance / 2.0);
                    }
                }

                let hue = f32(sample.index) / 10.0;
                let hsva = vec4<f32>(hue, 1.0, lightness, 1.0);
                let new_colour = hsva_to_rgba(hsva);

                textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y - 1), new_colour);
                break;
            }
        } else {
            break;
        }
    }



    let sample = trace(ray);

    if sample.hit {
        if sample.index == 0u {
            
        } else 
        {
            let hit_pos = ray.origin + (ray.direction * (sample.distance - 0.0001));
            let sun_dir = normalize(SUN_POSITION - hit_pos);
            let light_ray = Ray(hit_pos, sun_dir);

            let light_sample = trace(light_ray);
            
            var lightness = dot(light_ray.direction, sample.normal);

            if light_sample.hit {
                var colour = vec4<f32>(0.0, 0.0, 0.0, 1.0);
                if light_sample.distance < 2.0 {
                    lightness *= (light_sample.distance / 2.0) * (light_sample.distance / 2.0);
                }
            }

            let hue = f32(sample.index) / 10.0;
            let hsva = vec4<f32>(hue, 1.0, lightness, 1.0);
            let new_colour = hsva_to_rgba(hsva);

            textureStore(texture, vec2<i32>(pixel.x, i32(settings.height) - pixel.y - 1), new_colour);
        }
    }
}

fn trace(ray: Ray) -> Sample {
    var nearest_hit: f32 = 9999.0;
    var hit_something: bool = false;

    var sample = Sample(false, vec3<f32>(0.0, 0.0, 0.0), 0.0, 0u);

    var node: BVHNode = bvh_data[0];
    var stack: array<BVHNode, 15>;
    var stack_index: i32 = 0;

    while (true) {
        var count: u32 = u32(node.count);
        var contents: u32 = u32(node.left_child);

        if (count == 0u) {
            var child_1: BVHNode = bvh_data[contents];
            var child_2: BVHNode = bvh_data[contents + 1u];

            var distance_1 = hit_aabb(ray, child_1);
            var distance_2 = hit_aabb(ray, child_2);

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

                var new_sample = hit_triangle(
                    ray,
                    vec3<f32>(positions[tri_index.x].xyz),
                    vec3<f32>(positions[tri_index.y].xyz),
                    vec3<f32>(positions[tri_index.z].xyz),
                    u32(positions[tri_index.x].w)
                );

                if (new_sample.hit) {
                    hit_something = true;

                    if (new_sample.distance < nearest_hit) {
                        nearest_hit = new_sample.distance;
                        sample = new_sample;
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

    return sample;
}


fn hit_aabb(ray: Ray, node: BVHNode) -> f32 {
    var inverse_dir: vec3<f32> = vec3(1.0) / ray.direction;
    var t1: vec3<f32> = (node.mins - ray.origin) * inverse_dir;
    var t2: vec3<f32> = (node.maxs - ray.origin) * inverse_dir;
    var tMin: vec3<f32> = min(t1, t2);
    var tMax: vec3<f32> = max(t1, t2);

    var t_min: f32 = max(max(tMin.x, tMin.y), tMin.z);
    var t_max: f32 = min(min(tMax.x, tMax.y), tMax.z);

    if (t_min > t_max || t_max < 0.0) {
        return 9999.0;
    }
    return t_min;
}

fn hit_triangle(ray: Ray, p0: vec3<f32>, p1: vec3<f32>, p2: vec3<f32>, index: u32) -> Sample {
    let e2 = p2 - p0;
    let e1 = p1 - p0;
    let h = cross(ray.direction, e2);
    let a = dot(e1, h);

    if a > -0.00001 && a < 0.00001 {
        return Sample(false, vec3<f32>(0.0, 0.0, 0.0), 0.0, 0u);
    }

    let f = 1.0 / a;
    let s = ray.origin - p0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return Sample(false, vec3<f32>(0.0, 0.0, 0.0), 0.0, 0u);
    }

    let q = cross(s, e1);
    let v = f * dot(ray.direction, q);

    if v < 0.0 || u + v > 1.0 {
        return Sample(false, vec3<f32>(0.0, 0.0, 0.0), 0.0, 0u);
    }

    let t = f * dot(e2, q);

    if t > 0.00001 {
        let normal = normalize(cross(e1, e2));
        return Sample(true, normal, t, index);
    }

    return Sample(false, vec3<f32>(0.0, 0.0, 0.0), 0.0, 0u);
}
