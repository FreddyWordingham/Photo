const BINDINGS_DRAW: &str = include_str!("../shaders/bindings/draw.wgsl");

const COLOURS_UTILS: &str = include_str!("../shaders/utils/colours.wgsl");

const RAY: &str = include_str!("../shaders/geometry/ray.wgsl");
const BVH_NODE: &str = include_str!("../shaders/geometry/bounding_view_hierarchy.wgsl");

const SETTINGS_UNIFORM: &str = include_str!("../shaders/uniforms/settings.wgsl");
const CAMERA_UNIFORM: &str = include_str!("../shaders/uniforms/camera.wgsl");

const DISPLAY_MAIN: &str = include_str!("../shaders/display.wgsl");
const BACKGROUND_MAIN: &str = include_str!("../shaders/draw_background.wgsl");
const SCENE_MAIN: &str = include_str!("../shaders/draw_scene.wgsl");
const OBJECTS_MAIN: &str = include_str!("../shaders/draw_objects.wgsl");
const REFLECTIVE_OBJECTS_MAIN: &str = include_str!("../shaders/draw_reflective_objects.wgsl");
const NORMALS_MAIN: &str = include_str!("../shaders/draw_normals.wgsl");
const SMOOTH_NORMALS_MAIN: &str = include_str!("../shaders/draw_smooth_normals.wgsl");

pub struct ShaderProgram {}

impl ShaderProgram {
    pub fn display_shader() -> String {
        DISPLAY_MAIN.to_string()
    }

    pub fn draw_background() -> String {
        [
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            BACKGROUND_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_scene() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            SCENE_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_reflective_objects() -> String {
        [
            BINDINGS_DRAW,
            COLOURS_UTILS,
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            REFLECTIVE_OBJECTS_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_objects() -> String {
        [
            BINDINGS_DRAW,
            COLOURS_UTILS,
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            OBJECTS_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_normals() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            NORMALS_MAIN,
        ]
        .join("\n")
    }

    pub fn draw_smooth_normals() -> String {
        [
            BINDINGS_DRAW,
            BVH_NODE,
            RAY,
            SETTINGS_UNIFORM,
            CAMERA_UNIFORM,
            SMOOTH_NORMALS_MAIN,
        ]
        .join("\n")
    }
}
