use photo;
use pixels;
use winit;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;
const SCALE: u32 = 2;

fn main() {
    // Create the window.
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Photo Rendering")
        .with_inner_size(winit::dpi::LogicalSize::new(
            (WIDTH * SCALE) as f32,
            (HEIGHT * SCALE) as f32,
        ))
        .build(&event_loop)
        .unwrap();

    // Window texture.
    let mut window_texture = {
        let window_size = window.inner_size();
        let surface_texture =
            pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
        pixels::Pixels::new(WIDTH, HEIGHT, surface_texture)
            .expect("Failed to create pixels window texture")
    };

    // Program state.
    let mut canvas = photo::Canvas::new(WIDTH, HEIGHT);
    let mut state = photo::State::new();

    // World state.
    let mut world = photo::World::new();

    // Initialize the GPU.
    let hardware = pollster::block_on(photo::gpu::Hardware::new());
    let hardware = std::sync::Arc::new(hardware);

    // Load the shaders
    let invert = Invert::new(
        hardware.clone(),
        include_str!("add.wgsl"),
        world,
        (canvas
            .pixels
            .clone()
            .into_iter()
            .map(|col| col as f32 / 255.0))
        .collect::<Vec<f32>>(),
    );

    // Run the event loop.
    let mut prev_100_average_fps = 0.0;
    event_loop.run(move |event, _, control_flow| {
        // Process events.
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        photo::controls::handle_keypress(
                            keycode,
                            input.state,
                            control_flow,
                            &mut state,
                        );
                    }
                }
                _ => (),
            },
            _ => (),
        }

        // Update the state.
        world.player_x += 0.01;
        if world.player_x >= 1.0 as f32 {
            world.player_x = 0.0;
        }

        // Render the scene.
        canvas.clear_background([0, 0, 0, 255]);
        {
            let player_x = (world.player_x * WIDTH as f32) as u32;
            let player_y = (world.player_y * HEIGHT as f32) as u32;
            let radius = 5;
            let col = [255, 0, 0, 255];
            canvas.draw_circle(player_x, player_y, radius, col);
        }

        let time_now = std::time::Instant::now();
        let updated_pixels = pollster::block_on(
            invert.run(
                world,
                canvas
                    .pixels
                    .clone()
                    .into_iter()
                    .map(|col| col as f32 / 255.0)
                    .collect::<Vec<f32>>(),
            ),
        );
        canvas.pixels.copy_from_slice(
            updated_pixels
                .into_iter()
                .map(|col| (col * 255.0) as u8)
                .collect::<Vec<u8>>()
                .as_slice(),
        );
        // invert.apply(&mut canvas.pixels);
        let time_elapsed = time_now.elapsed();

        // println!("Time elapsed: {:?}", time_elapsed);
        let fps = 1.0 / (time_elapsed.as_secs_f32() + 1e-9);
        prev_100_average_fps = ((prev_100_average_fps * 99.0) + fps) / 100.0;
        println!("FPS: {}", prev_100_average_fps);

        // Update the window texture.
        canvas.render(window_texture.frame_mut());
        window_texture
            .render()
            .expect("Failed to render window texture");
    });
}

struct Invert {
    hardware: std::sync::Arc<photo::gpu::Hardware>,
    shaders: photo::gpu::ComputeShaderRunner,
}

impl Invert {
    fn new(
        hardware: std::sync::Arc<photo::gpu::Hardware>,
        shader_code: &str,
        uniform: photo::World,
        buffer_data: Vec<f32>,
    ) -> Self {
        let mut chunks: Vec<[f32; 4]> = buffer_data
            .chunks_exact(4)
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect();

        let shaders = photo::gpu::ComputeShaderRunner::new(
            &hardware,
            include_str!("add.wgsl"),
            uniform,
            &chunks,
        );

        Self { hardware, shaders }
    }

    pub async fn run(&self, uniform: photo::World, buffer_data: Vec<f32>) -> Vec<f32> {
        let mut chunks: Vec<[f32; 4]> = buffer_data
            .chunks_exact(4)
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect();

        // println!(" Input: {:?}", chunks[9]);
        chunks = self
            .shaders
            .run(&self.hardware, &uniform, &mut chunks)
            .await;
        // println!("Output: {:?}", chunks[9]);

        return chunks.iter().flatten().copied().collect();
    }
}

// Helper function to validate buffer size
fn validate_buffer_size<BufferData: bytemuck::Pod>(read_write_storage: &[BufferData]) -> usize {
    let buffer_size = std::mem::size_of_val(read_write_storage);
    if buffer_size % 16 != 0 {
        panic!("Buffer size must be a multiple of 16 bytes");
    }
    buffer_size
}

use wgpu::util::DeviceExt;

// Helper function to create a buffer
fn create_buffer<BufferData: bytemuck::Pod>(
    device: &wgpu::Device,
    label: &str,
    contents: &[BufferData],
    usage: wgpu::BufferUsages,
) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(contents),
        usage,
    })
}

// Helper function to create BindGroupEntry
fn create_bind_group_entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
        binding,
        resource: buffer.as_entire_binding(),
    }
}
