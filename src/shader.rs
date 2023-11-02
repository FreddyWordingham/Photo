const BINDINGS_DRAW: &str = include_str!("shaders/bindings/draw.wgsl");

const BVH_NODE: &str = include_str!("shaders/bvh/node.wgsl");

const SETTINGS_UNIFORM: &str = include_str!("shaders/uniforms/settings.wgsl");
const SETTINGS_CAMERA: &str = include_str!("shaders/uniforms/camera.wgsl");

const DISPLAY_MAIN: &str = include_str!("shaders/display.wgsl");
const BACKGROUND_MAIN: &str = include_str!("shaders/draw_background.wgsl");
const SCENE_MAIN: &str = include_str!("shaders/draw_scene.wgsl");
const NORMALS_MAIN: &str = include_str!("shaders/draw_normals.wgsl");
const SMOOTH_NORMALS_MAIN: &str = include_str!("shaders/draw_smooth_normals.wgsl");

pub struct Shader {}

impl Shader {
    pub fn display_shader() -> String {
        DISPLAY_MAIN.to_string()
    }

    pub fn draw_background() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            SETTINGS_UNIFORM,
            SETTINGS_CAMERA,
            BACKGROUND_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_scene() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            SETTINGS_UNIFORM,
            SETTINGS_CAMERA,
            SCENE_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_normals() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            SETTINGS_UNIFORM,
            SETTINGS_CAMERA,
            NORMALS_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_smooth_normals() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            SETTINGS_UNIFORM,
            SETTINGS_CAMERA,
            SMOOTH_NORMALS_MAIN,
        ]
        .join("\n")
    }
}
