@group(0) @binding(0) var screen_sampler: sampler;
@group(0) @binding(1) var colour_buffer: texture_2d<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coord: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let positions = array<vec2<f32>, 6>(
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(-1.0, 1.0)
    );

    let texture_coords = array<vec2<f32>, 6>(
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 0.0)
    );

    return VertexOutput(
        vec4<f32>(positions[vertex_index], 0.0, 1.0),
        texture_coords[vertex_index]
    );
}

@fragment
fn fs_main(@location(0) texture_coord: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(colour_buffer, screen_sampler, texture_coord);
}
