@group(0) @binding(0) var colour_buffer: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let screen_pos = vec2<i32>(i32(global_id.x), i32(global_id.y));
    var pixel_colour = vec3<f32>(0.0, 0.0, 0.0);
    textureStore(colour_buffer, screen_pos, vec4<f32>(pixel_colour, 1.0));
}
