use crate::{Camera, Hardware, Memory, Pipelines, Scene, Settings};

pub struct Render {
    pub hardware: Hardware,
    pub memory: Memory,
    pub pipelines: Pipelines,
    pub settings: Settings,
    pub camera: Camera,
}

impl Render {
    pub async fn new(
        resolution: [u32; 2],
        settings: Settings,
        camera: Camera,
        scene: Scene,
        window: winit::window::Window,
    ) -> Self {
        let hardware = Hardware::new(window).await;
        let memory = Memory::new(resolution, &settings, &camera, scene, &hardware.device);
        let pipelines = Pipelines::new(&hardware, &memory);

        Self {
            hardware,
            memory,
            pipelines,
            settings,
            camera,
        }
    }
}
