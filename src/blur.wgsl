struct Settings {
    x: f32,
    y: f32,
};

@group(0)
@binding(0)
var<uniform> settings: Settings;

@group(0)
@binding(1)
var read_texture: texture_storage_2d<rgba32float, read>;

@group(0)
@binding(2)
var write_texture: texture_storage_2d<rgba32float, write>;


@compute
@workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x = global_id.x;
    let y = global_id.y;

    let loc = vec2<u32>(x, y);

    let blur_radius: u32 = u32(3);
    let col = avg_colour_around(loc, blur_radius);

    textureStore(write_texture, loc, col);
}

fn avg_colour_around(centre: vec2<u32>, radius: u32) -> vec4<f32> {
    var sum: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var count: f32 = 0.0;

    let dims = textureDimensions(read_texture);
    let int_radius = i32(radius);
    let half_radius = radius / 2u;
    let int_centre = vec2<i32>(i32(centre.x), i32(centre.y));

    for (var dx: i32 = -int_radius; dx <= int_radius; dx = dx + 1) {
        for (var dy: i32 = -int_radius; dy <= int_radius; dy = dy + 1) {
            let d = dx * dx + dy * dy;
            if d > int_radius * int_radius {
                continue;
            }

            let pos = int_centre + vec2<i32>(dx, dy) - vec2<i32>(int_radius, int_radius);
            if (any(pos < vec2<i32>(0))) || any(pos >= vec2<i32>(dims)) {
                continue;
            }

            sum += textureLoad(read_texture, vec2<u32>(u32(pos.x), u32(pos.y)));
            count += 1.0;
        }
    }

    return sum / count;
}
