fn pseudo_random(u: vec2<f32>) -> f32 {
    let p = vec3<f32>(dot(u, vec2<f32>(127.1, 311.7)), dot(u, vec2<f32>(269.5, 183.3)), dot(u, vec2<f32>(419.2, 371.9)));

    return fract(sin(p.x) * 43758.5453 + sin(p.y) * 22786.5453 + sin(p.z) * 14761.5453);
}


// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) colour: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = vec4<f32>(model.position, 1.0);
    out.position = vec2<f32>(model.position.x, model.position.y);

    return out;
}


// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(pseudo_random(in.position), pseudo_random(in.position + vec2<f32>(0.0, 1.0)), pseudo_random(in.position + vec2<f32>(1.0, 0.0)), 1.0);
}
