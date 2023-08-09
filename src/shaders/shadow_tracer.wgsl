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
    camera_up: vec3<f32>,
    sphere_count: f32,
}

struct RenderState {
    distance: f32,
    normal: vec3<f32>,
    colour: vec3<f32>,
    hit: bool,
}

const SUN_POS: vec3<f32> = vec3<f32>(200.0, -300.0, 500.0);

const INFINITY: f32 = 9999999.9;
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

    let sample = sample_bvh(ray);

    var lightness = 1.0;
    if sample.hit {
        let surface_position = ray.origin + (sample.distance * ray.direction);
        let surface_to_sun = normalize(SUN_POS - surface_position);
        let surface_normal = sample.normal;
        let surface_to_sun_dot_normal = dot(surface_to_sun, surface_normal);
        lightness = max(0.0, surface_to_sun_dot_normal) * 0.9 + 0.1;
    }

    colour += sample.colour * lightness;


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

            let sample = sample_bvh(ray);

            colour += sample.colour;
        }
    }

    // Normalise the colour
    colour /= f32(SUPER_SAMPLES_SQRT * SUPER_SAMPLES_SQRT);
    return colour;
}

fn sample_bvh(ray: Ray) -> RenderState {
    var state = RenderState(
        INFINITY,
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(0.0, 0.0, 0.0),
        false
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
                    0.001,
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

        if t > min_distance && t < max_distance {
            return RenderState(t, normal, sphere.colour, true);
        }
    }

    return RenderState(old_state.distance, old_state.normal, old_state.colour, false);
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
