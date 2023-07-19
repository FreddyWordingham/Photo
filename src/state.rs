use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::{
    bind_group_controller::BindGroupController, camera::Camera,
    camera_controller::CameraController, camera_uniform::CameraUniform, texture::Texture, Vertex,
    INDICES_A, INDICES_B, VERTICES_A, VERTICES_B,
};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    clear_colour: wgpu::Color,

    bind_group_controller: BindGroupController,
    diffuse_bind_groups: Vec<wgpu::BindGroup>,
    diffuse_bind_group_index: usize,

    render_pipelines: Vec<wgpu::RenderPipeline>,
    render_pipeline_index: usize,
    model_buffers: Vec<(wgpu::Buffer, wgpu::Buffer, u32)>,
    model_index: usize,

    camera_controller: CameraController,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

impl State {
    /// Construct a new `State` from a `Window`.
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        // Textures
        let diffuse_texture_bytes_a = include_bytes!("resources/images/happy_tree.png");
        let diffuse_texture_a =
            Texture::from_bytes(&device, &queue, diffuse_texture_bytes_a, "Happy Tree").unwrap();
        let diffuse_texture_bytes_b = include_bytes!("resources/images/happy_tree-cartoon.png");
        let diffuse_texture_b = Texture::from_bytes(
            &device,
            &queue,
            diffuse_texture_bytes_b,
            "Happy Tree Cartoon",
        )
        .unwrap();

        // Bind texture.
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Texture Bind Group Layout"),
            });

        let create_bind_group = |label: &str, texture: &Texture| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
                ],
                label: Some(label),
            })
        };
        let diffuse_bind_group_a =
            create_bind_group("Happy Tree - Diffuse Bind Group", &diffuse_texture_a);
        let diffuse_bind_group_b = create_bind_group(
            "Happy Tree Cartoon - Diffuse Bind Group",
            &diffuse_texture_b,
        );

        // Camera
        let camera = Camera {
            position: cgmath::Point3::new(0.0, 1.0, 2.0),
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::unit_y(),
            aspect_ratio: config.width as f32 / config.height as f32,
            vertical_field_of_view: 45.0,
            near_clip: 0.1,
            far_clip: 100.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Camera Bind Group Layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        // Render Pipelines
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let make_pipeline = |shader| {
            return Self::create_render_pipeline_handle(
                &device,
                &config,
                shader,
                &render_pipeline_layout,
            );
        };

        let mut render_pipelines = Vec::with_capacity(3);
        render_pipelines.push(make_pipeline(wgpu::include_wgsl!("texture.wgsl")));
        render_pipelines.push(make_pipeline(wgpu::include_wgsl!("brown.wgsl")));
        render_pipelines.push(make_pipeline(wgpu::include_wgsl!("noise.wgsl")));
        render_pipelines.push(make_pipeline(wgpu::include_wgsl!("coloured.wgsl")));

        let mut model_buffers = Vec::with_capacity(2);
        let mut add_model_buffer = |vertices, indices| {
            model_buffers.push((
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                }),
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }),
                INDICES_A.len() as u32,
            ));
        };
        add_model_buffer(VERTICES_A, INDICES_A);
        add_model_buffer(VERTICES_B, INDICES_B);

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            clear_colour: wgpu::Color::WHITE,
            bind_group_controller: BindGroupController::new(),
            diffuse_bind_groups: vec![diffuse_bind_group_a, diffuse_bind_group_b],
            diffuse_bind_group_index: 0,
            render_pipelines,
            render_pipeline_index: 0,
            model_buffers,
            model_index: 0,
            camera_controller: CameraController::new(1.0),
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
        }
    }

    fn create_render_pipeline_handle(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: wgpu::ShaderModuleDescriptor,
        render_pipeline_layout: &wgpu::PipelineLayout,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(shader);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    /// Get a reference to the window.
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Set the clear colour.
    pub fn set_clear_colour(&mut self, colour: wgpu::Color) {
        self.clear_colour = colour;
    }

    /// Handle a window resize event by updating the surface configuration.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            return;
        }

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Return true if the event has fully been processed.
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        [
            self.bind_group_controller.process_events(event),
            self.camera_controller.process_events(event),
        ]
        .iter()
        .any(|&x| x)
    }

    pub fn update(&mut self) {
        self.bind_group_controller.update_bind_group_index(
            &mut self.diffuse_bind_group_index,
            &self.diffuse_bind_groups,
        );

        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    pub fn cycle_render_pipeline(&mut self) {
        self.render_pipeline_index = (self.render_pipeline_index + 1) % self.render_pipelines.len();
    }

    pub fn cycle_model(&mut self) {
        self.model_index = (self.model_index + 1) % self.model_buffers.len();
    }

    /// Render to the next frame.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_colour),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipelines[self.render_pipeline_index]);

            render_pass.set_bind_group(
                0,
                &self.diffuse_bind_groups[self.diffuse_bind_group_index],
                &[],
            );
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            let (vertex_buffer, index_buffer, num_indices) = &self.model_buffers[self.model_index];
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..*num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
