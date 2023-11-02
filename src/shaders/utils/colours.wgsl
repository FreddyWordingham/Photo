fn hsva_to_rgba(hsva: vec4<f32>) -> vec4<f32> {
    var r: f32 = 0.0;
    var g: f32 = 0.0;
    var b: f32 = 0.0;
    let i: u32 = u32(hsva.x * 6.0);
    let f: f32 = hsva.x * 6.0 - f32(i);
    let p: f32 = hsva.z * (1.0 - hsva.y);
    let q: f32 = hsva.z * (1.0 - f * hsva.y);
    let t: f32 = hsva.z * (1.0 - (1.0 - f) * hsva.y);

    switch (i % 6u) {
        case 0u: {
            r = hsva.z;
            g = t;
            b = p;
        }
        case 1u: {
            r = q;
            g = hsva.z;
            b = p;
        }
        case 2u: {
            r = p;
            g = hsva.z;
            b = t;
        }
        case 3u: {
            r = p;
            g = q;
            b = hsva.z;
        }
        case 4u: {
            r = t;
            g = p;
            b = hsva.z;
        }
        case 5u: {
            r = hsva.z;
            g = p;
            b = q;
        }
        default: {}
    }

    return vec4<f32>(r, g, b, hsva.a);
}
