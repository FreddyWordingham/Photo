struct Sphere {
    centre: vec3<f32>,
    radius: f32,
    colour: vec3<f32>,
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


@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> scene: Scene;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let screen_size = textureDimensions(colour_buffer);
    let screen_pos = vec2<i32>(i32(global_id.x), i32(global_id.y));

    let horizontal_coeff = (f32(screen_pos.x) - f32(screen_size.x) * 0.5) / f32(screen_size.x);
    let vertical_coeff = (f32(screen_pos.y) - f32(screen_size.y) * 0.5) / f32(screen_size.x);

    let direction = normalize(scene.camera_forward + horizontal_coeff * scene.camera_right + vertical_coeff * scene.camera_up);
    let ray_origin = scene.camera_position;
    let ray = Ray(ray_origin, direction);

    var pixel_colour = sample(ray, scene);
    textureStore(colour_buffer, screen_pos, vec4<f32>(pixel_colour, 1.0));
}

fn sample(ray: Ray, scene: Scene) -> vec3<f32> {
    return scene.camera_forward;
}

fn hit(ray: Ray, sphere: Sphere) -> bool {
    let oc = ray.origin - sphere.centre;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant > 0.0;
}
