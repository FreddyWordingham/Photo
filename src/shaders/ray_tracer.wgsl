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
    direction: vec3<f32>,
    origin: vec3<f32>,
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
    colour: vec3<f32>,
    hit: bool,
}

@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene: SceneData;
@group(0) @binding(2) var<storage, read> objects: ObjectData;
@group(0) @binding(3) var<storage, read> tree: BVH;
@group(0) @binding(4) var<storage, read> sphere_lookup_table: ObjectIndices;

@compute @workgroup_size(1,1,1)
fn main(@builtin(global_invocation_id) GlobalInvocationID: vec3<u32>) {

    let screen_size: vec2<i32> = vec2<i32>(textureDimensions(colour_buffer));
    let screen_pos: vec2<i32> = vec2<i32>(i32(GlobalInvocationID.x), i32(GlobalInvocationID.y));

    let horizontal_coefficient: f32 = (f32(screen_pos.x) - f32(screen_size.x) * 0.5) / f32(screen_size.x);
    let vertical_coefficient: f32 = (f32(screen_pos.y) - f32(screen_size.y) * 0.5) / f32(screen_size.x);

    let forwards: vec3<f32> = scene.camera_forwards;
    let right: vec3<f32> = scene.camera_right;
    let up: vec3<f32> = scene.camera_up;

    var ray: Ray;
    ray.direction = normalize(forwards + horizontal_coefficient * right + vertical_coefficient * up);
    ray.origin = scene.camera_position;

    let pixel_colour: vec3<f32> = sample_bvh(ray);

    textureStore(colour_buffer, screen_pos, vec4<f32>(pixel_colour, 1.0));
}

fn sample_bvh(ray: Ray) -> vec3<f32> {

    var colour: vec3<f32> = vec3(0.0, 0.0, 0.0);

    var nearest_hit: f32 = 99999.0;
    var hit_something: bool = false;

    var state: RenderState;

    var node: Node = tree.nodes[0];
    var stack: array<Node, 15>;
    var stack_location = u32(0);

    while true {

        var sphere_count: u32 = u32(node.sphere_count);
        var contents: u32 = u32(node.left_child);

        if sphere_count == u32(0) {
            var child1: Node = tree.nodes[contents];
            var child2: Node = tree.nodes[contents + u32(1)];

            var distance1: f32 = hit_aabb(ray, child1);
            var distance2: f32 = hit_aabb(ray, child2);
            if distance1 > distance2 {
                var temp_dist: f32 = distance1;
                distance1 = distance2;
                distance2 = temp_dist;

                var temp_child: Node = child1;
                child1 = child2;
                child2 = temp_child;
            }

            if distance1 > nearest_hit {
                if stack_location == u32(0) {
                    break;
                } else {
                    stack_location -= u32(1);
                    node = stack[stack_location];
                }
            } else {
                node = child1;
                if distance2 < nearest_hit {
                    stack[stack_location] = child2;
                    stack_location += u32(1);
                }
            }
        } else {
            for (var i = u32(0); i < sphere_count; i++) {

                var new_state: RenderState = hit_sphere(
                    ray,
                    objects.spheres[u32(sphere_lookup_table.sphere_indices[i + contents])],
                    0.001,
                    nearest_hit,
                    state
                );

                if new_state.hit {
                    nearest_hit = new_state.distance;
                    state = new_state;
                    hit_something = true;
                }
            }

            if stack_location == u32(0) {
                break;
            } else {
                stack_location -= u32(1);
                node = stack[stack_location];
            }
        }
    }

    if hit_something {
        colour = state.colour;
    }
    return colour;
}

fn hit_sphere(ray: Ray, sphere: Sphere, min_distance: f32, max_distance: f32, old_state: RenderState) -> RenderState {

    let co: vec3<f32> = ray.origin - sphere.center;
    let a: f32 = dot(ray.direction, ray.direction);
    let b: f32 = 2.0 * dot(ray.direction, co);
    let c: f32 = dot(co, co) - sphere.radius * sphere.radius;
    let discriminant: f32 = b * b - 4.0 * a * c;

    var state: RenderState;
    state.colour = old_state.colour;

    if discriminant > 0.0 {

        let t: f32 = (-b - sqrt(discriminant)) / (2.0 * a);

        if t > min_distance && t < max_distance {
            state.distance = t;
            state.colour = sphere.colour;
            state.hit = true;
            return state;
        }
    }

    state.hit = false;
    return state;
}

fn hit_aabb(ray: Ray, node: Node) -> f32 {
    var inverseDir: vec3<f32> = vec3(1.0) / ray.direction;
    var t1: vec3<f32> = (node.min_corner - ray.origin) * inverseDir;
    var t2: vec3<f32> = (node.max_corner - ray.origin) * inverseDir;
    var min_distance: vec3<f32> = min(t1, t2);
    var max_distance: vec3<f32> = max(t1, t2);

    var t_min: f32 = max(max(min_distance.x, min_distance.y), min_distance.z);
    var t_max: f32 = min(min(max_distance.x, max_distance.y), max_distance.z);

    if t_min > t_max || t_max < 0.0 {
        return 99999.0;
    } else {
        return t_min;
    }
}
