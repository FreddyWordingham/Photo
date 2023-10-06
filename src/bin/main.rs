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
        include_str!("invert.wgsl"),
        world,
        &canvas
            .pixels
            .chunks(4)
            .map(|c| {
                [
                    c[0] as f32 / 255.0,
                    c[1] as f32 / 255.0,
                    c[2] as f32 / 255.0,
                    c[3] as f32 / 255.0,
                ]
            })
            .collect::<Vec<_>>(),
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
        invert.apply(&mut canvas.pixels);
        let time_elapsed = time_now.elapsed();

        println!("Time elapsed: {:?}", time_elapsed);
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
    _hardware: std::sync::Arc<photo::gpu::Hardware>,
    _buffer_size: usize,
    _uniform_buffer: wgpu::Buffer,
    _storage_buffer: wgpu::Buffer,
    _copy_buffer: wgpu::Buffer,
    _compute_pipeline: wgpu::ComputePipeline,
    _bind_group: wgpu::BindGroup,
}

impl Invert {
    fn new(
        hardware: std::sync::Arc<photo::gpu::Hardware>,
        shader_code: &str,
        uniform: photo::World,
        buffer_data: &[[f32; 4]],
    ) -> Self {
        let buffer_size = validate_buffer_size(buffer_data);

        let uniform_buffer = create_buffer(
            hardware.device(),
            "Uniform Buffer",
            &[uniform],
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        );

        let storage_buffer = create_buffer(
            hardware.device(),
            "Storage Buffer",
            buffer_data,
            wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
        );
        // Instantiates buffer without data.
        let copy_buffer = hardware.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size as u64,
            usage: wgpu::BufferUsages::MAP_READ | // `MAP_READ` allows it to be read (outside the shader).
                    wgpu::BufferUsages::COPY_DST, // `COPY_DST` allows it to be the destination of the copy.
            mapped_at_creation: false,
        });

        // Load the shaders
        let shader_module = hardware
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader_code)),
            });

        // Instantiates the pipeline.
        let compute_pipeline =
            hardware
                .device()
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &shader_module,
                    entry_point: "main",
                });

        // Instantiates the bind groups, once again specifying the binding of buffers.
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = hardware
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    create_bind_group_entry(0, &uniform_buffer),
                    create_bind_group_entry(1, &storage_buffer),
                ],
            });

        Self {
            _hardware: hardware,
            _buffer_size: buffer_size,
            _uniform_buffer: uniform_buffer,
            _storage_buffer: storage_buffer,
            _copy_buffer: copy_buffer,
            _compute_pipeline: compute_pipeline,
            _bind_group: bind_group,
        }
    }

    fn apply(&self, pixels: &mut [u8]) {
        for col in pixels.chunks_mut(4) {
            col[0] = 255 - col[0];
            col[1] = 255 - col[1];
            col[2] = 255 - col[2];
            col[3] = col[3];
        }
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
