@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var<uniform> camera: Camera;

@group(0)
@binding(2)
var texture: texture_storage_2d<rgba8unorm, read_write>;

@group(0)
@binding(3)
var<storage, read> positions: array<vec4<f32>>;

@group(0)
@binding(4)
var<storage, read> normals: array<vec4<f32>>;

@group(0)
@binding(5)
var<storage, read> position_indices: array<vec3<u32>>;

@group(0)
@binding(6)
var<storage, read> normal_indices: array<vec3<u32>>;

@group(0)
@binding(7)
var<storage, read> bvh_data: array<BVHNode>;

@group(0)
@binding(8)
var<storage, read> bvh_indices: array<u32>;
