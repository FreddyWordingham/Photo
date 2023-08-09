struct Sphere {
    centre: vec3<f32>,
    colour: vec3<f32>,
    radius: f32,
}

struct ObjectData {
    spheres: array<Sphere>,
}

struct Node {
    min: vec3<f32>,
    left_child: f32,
    max: vec3<f32>,
    sphere_count: f32,
}

struct BVH {
    nodes: array<Node>,
}

struct ObjectIndicies {
    sphere_indicies: array<u32>,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct Scene {
    camera_position: vec3<f32>,
    camera_forward: vec3<f32>,
    camera_right: vec3<f32>,
    camera_up: vec3<f32>,
    sphere_count: f32,
}

struct RenderState {
    distance: f32,
    colour: vec3<f32>,
    hit: bool,
}


@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene: Scene;
@group(0) @binding(2) var<storage, read> objects: ObjectData;
@group(0) @binding(3) var<storage, read> bvh: BVH;
@group(0) @binding(4) var<storage, read> sphere_lookup: ObjectIndicies;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let screen_size = textureDimensions(colour_buffer);
    let inv_screen_width = 1.0 / f32(screen_size.x);
    let half_screen_size = 0.5 * vec2<f32>(f32(screen_size.x), f32(screen_size.y));

    const min_distance = 0.001;
    const max_distance = 100000.0;
    // const num_samples_sqrt = 2;

    // var r = 0.0;
    // var g = 0.0;
    // var b = 0.0;
    // for (var u = 0; u < num_samples_sqrt; u++) {
    //     let du = f32(u) / f32(num_samples_sqrt);
    //     for (var v = 0; v < num_samples_sqrt; v++) {
    //         let dv = f32(v) / f32(num_samples_sqrt) ;
    //         let screen_pos = vec2<f32>(f32(global_id.x) + du, f32(global_id.y) + dv);

    //         let horizontal_coeff = (screen_pos.x - half_screen_size.x) * inv_screen_width;
    //         let vertical_coeff = (screen_pos.y - half_screen_size.y) * inv_screen_width;

    //         let direction = normalize(scene.camera_forward + horizontal_coeff * scene.camera_right + vertical_coeff * scene.camera_up);
    //         let ray = Ray(scene.camera_position, direction);

    //         let sample = sample(ray, scene, min_distance, max_distance);
    //         let pixel_colour = sample.colour;
    //         r += pixel_colour.r;
    //         g += pixel_colour.g;
    //         b += pixel_colour.b;
    //     }
    // }
    // r /= f32(num_samples_sqrt * num_samples_sqrt);
    // g /= f32(num_samples_sqrt * num_samples_sqrt);
    // b /= f32(num_samples_sqrt * num_samples_sqrt);

    // textureStore(colour_buffer, vec2<i32>(i32(global_id.x), i32(global_id.y)), vec4<f32>(r, g, b, 1.0));

    let screen_pos = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let horizontal_coeff = (screen_pos.x - half_screen_size.x) * inv_screen_width;
    let vertical_coeff = (screen_pos.y - half_screen_size.y) * inv_screen_width;
    let direction = normalize(scene.camera_forward + horizontal_coeff * scene.camera_right + vertical_coeff * scene.camera_up);
    let ray = Ray(scene.camera_position, direction);

    // let sample = simple_sample(ray, scene, min_distance, max_distance);
    let sample = bvh_sample(ray, scene, min_distance, max_distance);

    textureStore(colour_buffer, vec2<i32>(i32(global_id.x), i32(global_id.y)), vec4<f32>(sample.colour, 1.0));
}

// Sample the scene for the nearest hit
fn simple_sample(ray: Ray, scene: Scene, min_distance: f32, max_distance: f32) -> RenderState {
    // Create default return state
    var nearest_hit = RenderState(max_distance, vec3<f32>(0.0, 0.0, 0.0), false);
    // For each sphere
    for (var n: u32 = u32(0); n < u32(scene.sphere_count); n++) {
        // Check if the ray hits the sphere
        let new_hit = hit_sphere(ray, objects.spheres[n], min_distance, nearest_hit.distance, nearest_hit);
        // If the ray hits the sphere, update the state with the new state
        // (We already know that it is closer than the previous state)
        if new_hit.hit { nearest_hit = new_hit; }
    }
    // Return the colour the nearest hit
    return nearest_hit;
}

// Sample the scene for the nearest hit
fn bvh_sample(ray: Ray, scene: Scene, min_distance: f32, max_distance: f32) -> RenderState {
    // Create default return state
    var nearest_hit = RenderState(max_distance, vec3<f32>(0.0, 0.0, 0.0), false);

    // Track the node
    var node: Node = bvh.nodes[0];
    var stack: array<Node, 15>;
    var stack_index: u32 = u32(0);
    while true {
        let sphere_count: u32 = u32(node.sphere_count);
        let contents: u32 = u32(node.left_child);

        if sphere_count == u32(0) {
            var child1: Node = bvh.nodes[contents];
            var child2: Node = bvh.nodes[contents + u32(1)];

            var distance1 = hit_aabb(ray, child1);
            var distance2 = hit_aabb(ray, child2);

            if distance1 > distance2 {
                let temp_distance: f32 = distance1;
                distance1 = distance2;
                distance2 = temp_distance;

                let temp_node: Node = child1;
                child1 = child2;
                child2 = temp_node;
            }

            if distance1 > nearest_hit.distance {
                if stack_index == u32(0) {
                    break;
                } else {
                    stack_index -= u32(1);
                    node = stack[stack_index];
                }
            } else {
                node = child1;
                if distance2 < nearest_hit.distance {
                    stack[stack_index] = child2;
                    stack_index += u32(1);
                }
            }
        } else {
            // // For each sphere
            // for (var n: u32 = u32(0); n < sphere_count; n++) {
            //     // Check if the ray hits the sphere
            //     let sphere_index = sphere_lookup.sphere_indicies[contents + n];
            //     let new_hit = hit_sphere(ray, objects.spheres[sphere_index], min_distance, nearest_hit.distance, nearest_hit);
            //     // If the ray hits the sphere, update the state with the new state
            //     // (We already know that it is closer than the previous state)
            //     if new_hit.hit { nearest_hit = new_hit; }
            // }

            // For each sphere
            for (var n: u32 = u32(0); n < u32(sphere_count); n++) {
                // Check if the ray hits the sphere
                let sphere_index = sphere_lookup.sphere_indicies[contents + n];
                let new_hit = hit_sphere(ray, objects.spheres[sphere_index], min_distance, nearest_hit.distance, nearest_hit);
                // If the ray hits the sphere, update the state with the new state
                // (We already know that it is closer than the previous state)
                if new_hit.hit { nearest_hit = new_hit; }
            }

            if stack_index == u32(0) {
                break;
            } else {
                stack_index -= u32(1);
                node = stack[stack_index];
            }
        }
    }

    // Return the colour the nearest hit
    return nearest_hit;
}

fn hit_sphere(ray: Ray, sphere: Sphere, min_distance: f32, max_distance: f32, state: RenderState) -> RenderState {
    let oc = ray.origin - sphere.centre;
    let b = dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - c;

    if discriminant > 0.0 {
        let temp = (-b - sqrt(discriminant)) * 0.5;
        if temp < max_distance && temp > min_distance {
            // let point = ray.origin + temp * ray.direction;
            // let normal = (point - sphere.centre) / sphere.radius;
            let colour = sphere.colour;
            return RenderState(temp, colour, true);
        }
    }
    return state;
}

fn hit_aabb(ray: Ray, node: Node) -> f32 {
    var inv_direction = vec3(1.0) / ray.direction;
    var t1: vec3<f32> = (node.min - ray.origin) * inv_direction;
    var t2: vec3<f32> = (node.max - ray.origin) * inv_direction;

    var min_distance: vec3<f32> = min(t1, t2);
    var max_distance: vec3<f32> = max(t1, t2);

    var closest = max(max(min_distance.x, min_distance.y), min_distance.z);
    var furthest = min(min(max_distance.x, max_distance.y), max_distance.z);

    if closest > furthest || furthest < 0.0 {
        return 99999.0;
    } else {
        return closest;
    }
}
