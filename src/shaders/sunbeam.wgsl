struct Sphere {
    center: vec3<f32>,
    colour: vec3<f32>,
    radius: f32,
}

struct ObjectData {
    spheres: array<Sphere>,
}

struct Node {
    min_corner: vec3<f32>,
    left_child: f32,
    max_corner: vec3<f32>,
    sphere_count: f32,
}

struct BVH {
    nodes: array<Node>,
}

struct ObjectIndices {
    sphere_indices: array<f32>,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct SceneData {
    camera_position: vec3<f32>,
    camera_forwards: vec3<f32>,
    camera_right: vec3<f32>,
    max_bounces: f32,
    camera_up: vec3<f32>,
    sphere_count: f32,
}

struct RenderState {
    distance: f32,
    colour: vec3<f32>,
    hit: bool,
    normal: vec3<f32>,
}

const SUN_POS: vec3<f32> = vec3<f32>(0.0, -0.0, 0.0);

const INFINITY: f32 = 1000.0;
const EPSILON: f32 = 0.001;
const MAX_GAS_SAMPLES: i32 = 45;
const STEP: f32 = 300.0 / f32(MAX_GAS_SAMPLES);

const SUPER_SAMPLES_SQRT: u32 = 2;

@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene: SceneData;
@group(0) @binding(2) var<storage, read> objects: ObjectData;
@group(0) @binding(3) var<storage, read> tree: BVH;
@group(0) @binding(4) var<storage, read> sphere_lookup_table: ObjectIndices;

@compute @workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {

    let screen_size = vec2<i32>(textureDimensions(colour_buffer));
    let screen_position = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));

    let forwards = scene.camera_forwards;
    let right = scene.camera_right;
    let up = scene.camera_up;

    let pixel_colour = single_sample(screen_size, screen_position, forwards, right, up);
    // let pixel_colour = multi_sample(screen_size, screen_position, forwards, right, up);

    // Write the colour to the buffer
    textureStore(colour_buffer, screen_position, vec4<f32>(pixel_colour, 1.0));
}

fn single_sample(
    screen_size: vec2<i32>,
    screen_position: vec2<i32>,
    forwards: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>
) -> vec3<f32> {
    var colour = vec3<f32>(0.0, 0.0, 0.0);

    let horizontal_coefficient = (f32(screen_position.x) - f32(screen_size.x) * 0.5) / f32(screen_size.x);
    let vertical_coefficient = (f32(screen_position.y) - f32(screen_size.y) * 0.5) / f32(screen_size.x);

    let direction = normalize(forwards + (horizontal_coefficient) * right + (vertical_coefficient) * up);
    let origin = scene.camera_position;
    let ray = Ray(origin, direction);

    let sample = sample_bvh(ray, EPSILON, INFINITY);

    var gas = 0.0;
    let steps = min(MAX_GAS_SAMPLES, i32(sample.distance / STEP));
    for (var i = 0; i < steps; i++) {
        let sample_pos = ray.origin + (f32(i) * STEP * ray.direction);
        let sun_dir = normalize(SUN_POS - sample_pos);
        let gas_ray = Ray(sample_pos, sun_dir);
        let sun_dist = distance(SUN_POS, sample_pos);
        let gas_sample = sample_bvh(gas_ray, EPSILON, sun_dist);
        if !gas_sample.hit {
            gas += 45.0 / sun_dist;
        }
    }
    gas /= f32(MAX_GAS_SAMPLES);
    gas = clamp(gas, 0.0, 1.0);
    gas = pow(gas, 2.0);

    var lightness = 1.0;
    if sample.hit {
        let surface_position = ray.origin + (sample.distance * ray.direction);
        let surface_to_sun = normalize(SUN_POS - surface_position);
        let surface_normal = sample.normal;
        let surface_to_sun_dot_normal = dot(surface_to_sun, surface_normal);
        lightness = max(0.0, surface_to_sun_dot_normal);
        lightness = pow(lightness, 2.0);
    }

    var darkness = 1.0;
    if sample.hit {
        let surface_position = travel(ray, sample.distance);
        var surface_ray = Ray(surface_position, sample.normal);
        surface_ray.origin = travel(surface_ray, 0.01);
        let surface_to_sun = normalize(SUN_POS - surface_ray.origin);
        let sun_distance = distance(SUN_POS, surface_ray.origin);
        let shadow_ray = Ray(surface_ray.origin, surface_to_sun);
        let shadow_sample = sample_bvh(shadow_ray, EPSILON, sun_distance);
        if shadow_sample.hit {
            darkness = 0.0;
        }
    }

    var hsv = rgb_to_hsv(sample.colour);
    hsv.x = (hsv.x + (lightness * 60.0)) % 360.0;

    colour = hsv_to_rgb(hsv) * darkness;

    colour = (colour * 0.5) + (0.5 * vec3<f32>(gas, gas, gas));


    return colour;
}

fn multi_sample(
    screen_size: vec2<i32>,
    screen_position: vec2<i32>,
    forwards: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>
) -> vec3<f32> {
    var colour = vec3<f32>(0.0, 0.0, 0.0);

    for (var i = u32(0); i < SUPER_SAMPLES_SQRT; i++) {
        let du = f32(i) / f32(SUPER_SAMPLES_SQRT);
        let horizontal_coefficient = (du + f32(screen_position.x) - f32(screen_size.x) * 0.5) / f32(screen_size.x);
        for (var j = u32(0); j < SUPER_SAMPLES_SQRT; j++) {
            let dv = f32(j) / f32(SUPER_SAMPLES_SQRT);
            let vertical_coefficient = (dv + f32(screen_position.y) - f32(screen_size.y) * 0.5) / f32(screen_size.x);

            let direction = normalize(forwards + (horizontal_coefficient) * right + (vertical_coefficient) * up);
            let origin = scene.camera_position;
            let ray = Ray(origin, direction);

            // let sample = sample_bvh(ray, EPSILON, INFINITY);
            let sample = sample_bvh(ray, EPSILON, INFINITY);

            colour += sample.colour;
        }
    }

    // Normalise the colour
    colour /= f32(SUPER_SAMPLES_SQRT * SUPER_SAMPLES_SQRT);
    return colour;
}

fn sample_bvh(ray: Ray, min_distance: f32, max_distance: f32) -> RenderState {
    var state = RenderState(
        max_distance,
        vec3<f32>(0.0, 0.0, 0.0),
        false,
        vec3<f32>(0.0, 0.0, 1.0),
    );

    var node: Node = tree.nodes[0];
    var stack = array<Node, 15>();
    var stack_location = u32(0);

    while true {
        let sphere_count = u32(node.sphere_count);
        let contents = u32(node.left_child);

        if sphere_count == u32(0) {
            var child1: Node = tree.nodes[contents];
            var child2: Node = tree.nodes[contents + u32(1)];

            var distance1: f32 = hit_aabb(ray, child1);
            var distance2: f32 = hit_aabb(ray, child2);

            if distance1 > distance2 {
                let temp_dist: f32 = distance1;
                distance1 = distance2;
                distance2 = temp_dist;

                let temp_child: Node = child1;
                child1 = child2;
                child2 = temp_child;
            }

            if distance1 > state.distance {
                if stack_location == u32(0) { break; }
                stack_location -= u32(1);
                node = stack[stack_location];
            } else {
                node = child1;
                if distance2 < state.distance {
                    stack[stack_location] = child2;
                    stack_location += u32(1);
                }
            }
        } else {
            for (var i = u32(0); i < sphere_count; i++) {
                let new_state: RenderState = hit_sphere(
                    ray,
                    objects.spheres[u32(sphere_lookup_table.sphere_indices[i + contents])],
                    EPSILON,
                    state.distance,
                    state
                );

                if new_state.hit {
                    state = new_state;
                }
            }

            if stack_location == u32(0) { break; }
            stack_location -= u32(1);
            node = stack[stack_location];
        }
    }

    return state;
}

fn hit_sphere(ray: Ray, sphere: Sphere, min_distance: f32, max_distance: f32, old_state: RenderState) -> RenderState {

    let co: vec3<f32> = ray.origin - sphere.center;
    let half_b: f32 = dot(ray.direction, co);
    let c: f32 = dot(co, co) - sphere.radius * sphere.radius;
    let discriminant: f32 = half_b * half_b - c;

    if discriminant > 0.0 {
        let t: f32 = (-half_b - sqrt(discriminant));

        let position = ray.origin + t * ray.direction;
        let normal = normalize(position - sphere.center);

        if t >= min_distance && t <= max_distance {
            return RenderState(t, sphere.colour, true, normal);
        }
    }

    return RenderState(old_state.distance, old_state.colour, false, old_state.normal);
}

fn hit_aabb(ray: Ray, node: Node) -> f32 {
    let inverse_direction = vec3(1.0) / ray.direction;

    let t1 = (node.min_corner - ray.origin) * inverse_direction;
    let t2 = (node.max_corner - ray.origin) * inverse_direction;

    let min_distances = min(t1, t2);
    let max_distances = max(t1, t2);

    let nearest_intersection = max(max(min_distances.x, min_distances.y), min_distances.z);
    let furthest_intersection = min(min(max_distances.x, max_distances.y), max_distances.z);

    if furthest_intersection < 0.0 || nearest_intersection > furthest_intersection {
        return INFINITY;
    } else {
        return nearest_intersection;
    }
}

fn travel(ray: Ray, distance: f32) -> vec3<f32> {
    return ray.origin + (ray.direction * distance);
}


fn rgb_to_hsv(rgb: vec3<f32>) -> vec3<f32> {
    let cmax = max(rgb.r, max(rgb.g, rgb.b));
    let cmin = min(rgb.r, min(rgb.g, rgb.b));
    let delta = cmax - cmin;

    // Hue calculation
    var hue: f32 = 0.0;
    if delta == 0.0 {
        hue = 0.0;
    } else if cmax == rgb.r {
        hue = 60.0 * (((rgb.g - rgb.b) / delta) % 6.0);
    } else if cmax == rgb.g {
        hue = 60.0 * (((rgb.b - rgb.r) / delta) + 2.0);
    } else if cmax == rgb.b {
        hue = 60.0 * (((rgb.r - rgb.g) / delta) + 4.0);
    }

    if hue < 0.0 {
        hue += 360.0;
    }

    // Saturation calculation
    var saturation: f32 = 0.0;
    if cmax != 0.0 {
        saturation = delta / cmax;
    }

    // Value calculation
    let value = cmax;

    return vec3<f32>(hue, saturation, value);
}

fn hsv_to_rgb(hsv: vec3<f32>) -> vec3<f32> {
    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    let c = v * s;
    let x = c * (1.0 - abs((h / 60.0) - 2.0 * floor(h / 120.0) - 1.0));
    let m = v - c;

    var rp: f32 = 0.0;
    var gp: f32 = 0.0;
    var bp: f32 = 0.0;

    if h >= 0.0 && h < 60.0 {
        rp = c;
        gp = x;
        bp = 0.0;
    } else if h >= 60.0 && h < 120.0 {
        rp = x;
        gp = c;
        bp = 0.0;
    } else if h >= 120.0 && h < 180.0 {
        rp = 0.0;
        gp = c;
        bp = x;
    } else if h >= 180.0 && h < 240.0 {
        rp = 0.0;
        gp = x;
        bp = c;
    } else if h >= 240.0 && h < 300.0 {
        rp = x;
        gp = 0.0;
        bp = c;
    } else if h >= 300.0 && h < 360.0 {
        rp = c;
        gp = 0.0;
        bp = x;
    }

    return vec3<f32>(rp + m, gp + m, bp + m);
}
