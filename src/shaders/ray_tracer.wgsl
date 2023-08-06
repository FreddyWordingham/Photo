struct Sphere {
    centre: vec3<f32>,
    colour: vec3<f32>,
    radius: f32,
}

struct ObjectData {
    spheres: array<Sphere>,
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
    t: f32,
    colour: vec3<f32>,
    hit: bool,
}


@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene: Scene;
@group(0) @binding(2) var<storage, read> objects: ObjectData;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let screen_size = textureDimensions(colour_buffer);
    let inv_screen_width = 1.0 / f32(screen_size.x);
    let half_screen_size = 0.5 * vec2<f32>(f32(screen_size.x), f32(screen_size.y));

    let screen_pos = vec2<f32>(f32(global_id.x), f32(global_id.y));
    let horizontal_coeff = (screen_pos.x - half_screen_size.x) * inv_screen_width;
    let vertical_coeff = (screen_pos.y - half_screen_size.y) * inv_screen_width;

    let direction = normalize(scene.camera_forward + horizontal_coeff * scene.camera_right + vertical_coeff * scene.camera_up);
    let ray = Ray(scene.camera_position, direction);

    const min_distance: f32 = 0.001;
    const max_distance: f32 = 1000000.0;
    let sample = sample(ray, scene, min_distance, max_distance);
    let pixel_colour = sample.colour;
    textureStore(colour_buffer, vec2<i32>(i32(global_id.x), i32(global_id.y)), vec4<f32>(pixel_colour, 1.0));
}

fn sample(ray: Ray, scene: Scene, t_min: f32, t_max: f32) -> RenderState {
    // Create default return state
    var nearest_hit = RenderState(t_max, vec3<f32>(0.0, 0.0, 0.0), false);
    // For each sphere
    for (var n: u32 = u32(0); n < u32(scene.sphere_count); n++) {
        // Check if the ray hits the sphere
        let new_hit = hit(ray, objects.spheres[n], t_min, state.t, nearest_hit);
        // If the ray hits the sphere, update the state with the new state
        // (We already know that it is closer than the previous state)
        if new_hit.hit { nearest_hit = new_hit; }
    }
    // Return the colour the nearest hit
    return nearest_hit;
}

fn hit(ray: Ray, sphere: Sphere, t_min: f32, t_max: f32, state: RenderState) -> RenderState {
    let oc = ray.origin - sphere.centre;
    let b = dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - c;

    if discriminant > 0.0 {
        let temp = (-b - sqrt(discriminant)) / 2.0;
        if temp < t_max && temp > t_min {
            let point = ray.origin + temp * ray.direction;
            let normal = (point - sphere.centre) / sphere.radius;
            let colour = sphere.colour;
            return RenderState(temp, colour, true);
        }
    }
    return state;
}
