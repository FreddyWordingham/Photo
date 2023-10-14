use wgpu::util::DeviceExt;

use crate::{simulation::NBodyInit, Hardware, Image};

pub struct NBody {
    num_massive_bodies: usize,
    num_ghost_bodies: usize,
    hardware: Hardware,
    settings_buffer: wgpu::Buffer,
    texture_extent: wgpu::Extent3d,
    textures: [wgpu::Texture; 2],
    cpu_texture_buffer: wgpu::Buffer,

    calculate_ghost_forces_pipeline: wgpu::ComputePipeline,
    calculate_ghost_forces_bind_group: wgpu::BindGroup,
    calculate_ghost_velocities_pipeline: wgpu::ComputePipeline,
    calculate_ghost_velocities_bind_group: wgpu::BindGroup,
    calculate_ghost_positions_pipeline: wgpu::ComputePipeline,
    calculate_ghost_positions_bind_group: wgpu::BindGroup,
    render_ghost_positions_pipeline: wgpu::ComputePipeline,
    render_ghost_positions_bind_group: wgpu::BindGroup,
    render_massive_positions_pipeline: wgpu::ComputePipeline,
    render_massive_positions_bind_group: wgpu::BindGroup,
}

impl NBody {
    pub async fn new(
        init_conditions: NBodyInit,
        nrows: u32,
        ncols: u32,
        grav_force: f32,
        zoom: f32,
    ) -> Self {
        debug_assert!(init_conditions.is_valid());
        debug_assert!(ncols > 0);
        debug_assert!(nrows > 0);
        debug_assert!(grav_force > 0.0);
        debug_assert!(zoom > 0.0);

        // Used to determine buffer sizes
        let num_massive_bodies = init_conditions.num_massive_bodies();
        let num_ghost_bodies = init_conditions.num_ghost_bodies();

        // Initialise the GPU
        let hardware = Hardware::new().await;

        // Textures
        let texture_extent = create_texture_extent(nrows, ncols);
        let texture_a = create_texture(&hardware, &texture_extent);
        let texture_b = create_texture(&hardware, &texture_extent);
        let textures = [texture_a, texture_b];
        let cpu_texture_buffer = create_cpu_texture_buffer(&hardware, &texture_extent);

        // Settings
        let init_settings = [ncols as f32, nrows as f32, grav_force, zoom];
        let settings_buffer = create_settings_buffer(&hardware, &init_settings);

        // Ghost bodies
        let ghost_positions = create_4d_buffer(
            &hardware,
            init_conditions
                .ghost_positions
                .iter()
                .map(|[px, py, pz]| [*px, *py, *pz, 1.0])
                .collect::<Vec<_>>()
                .as_slice(),
        );
        let ghost_velocities = create_4d_buffer(
            &hardware,
            init_conditions
                .ghost_velocities
                .iter()
                .map(|[vx, vy, vz]| [*vx, *vy, *vz, 1.0])
                .collect::<Vec<_>>()
                .as_slice(),
        );
        let ghost_forces = create_4d_buffer(&hardware, &vec![[0.0; 4]; num_ghost_bodies]);

        // Massive bodies
        let massive_positions = create_4d_buffer(
            &hardware,
            init_conditions
                .massive_positions
                .iter()
                .zip(&init_conditions.massive_masses)
                .map(|([px, py, pz], m)| [*px, *py, *pz, *m])
                .collect::<Vec<_>>()
                .as_slice(),
        );

        // Pipelines and bind groups
        let (calculate_ghost_forces_pipeline, calculate_ghost_forces_bind_group) =
            create_calculate_ghost_forces_pipeline_and_bind_group(
                &hardware,
                &settings_buffer,
                &ghost_positions,
                &massive_positions,
                &ghost_forces,
            );

        let (calculate_ghost_velocities_pipeline, calculate_ghost_velocities_bind_group) =
            create_calculate_ghost_velocities_pipeline_and_bind_group(
                &hardware,
                &settings_buffer,
                &ghost_velocities,
                &ghost_forces,
            );

        let (calculate_ghost_positions_pipeline, calculate_ghost_positions_bind_group) =
            create_calculate_ghost_positions_pipeline_and_bind_group(
                &hardware,
                &settings_buffer,
                &ghost_positions,
                &ghost_velocities,
            );

        let (render_ghost_positions_pipeline, render_ghost_positions_bind_group) =
            create_render_ghost_positions_pipeline_and_bind_group(
                &hardware,
                &settings_buffer,
                &ghost_positions,
                &textures,
            );

        let (render_massive_positions_pipeline, render_massive_positions_bind_group) =
            create_render_massive_positions_pipeline_and_bind_group(
                &hardware,
                &settings_buffer,
                &massive_positions,
                &textures,
            );

        Self {
            num_massive_bodies,
            num_ghost_bodies,
            hardware,
            settings_buffer,
            texture_extent,
            textures,
            cpu_texture_buffer,
            calculate_ghost_forces_pipeline,
            calculate_ghost_forces_bind_group,
            calculate_ghost_velocities_pipeline,
            calculate_ghost_velocities_bind_group,
            calculate_ghost_positions_pipeline,
            calculate_ghost_positions_bind_group,
            render_ghost_positions_pipeline,
            render_ghost_positions_bind_group,
            render_massive_positions_pipeline,
            render_massive_positions_bind_group,
        }
    }

    pub fn set_settings(&mut self, settings: &[f32; 4]) {
        self.hardware.queue().write_buffer(
            &self.settings_buffer,
            0,
            bytemuck::cast_slice(settings),
        );
    }

    pub async fn run(&self, image: &mut Image) {
        let mut encoder =
            self.hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("N-Body - Command Encoder"),
                });

        {
            let view = &self.textures[0].create_view(&wgpu::TextureViewDescriptor::default());
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("N-Body - Clear Texture"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Calculate Ghost Forces"),
            });
            compute_pass.set_bind_group(0, &self.calculate_ghost_forces_bind_group, &[]);
            compute_pass.set_pipeline(&self.calculate_ghost_forces_pipeline);
            compute_pass.dispatch_workgroups((self.num_ghost_bodies / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Calculate Ghost Velocities"),
            });
            compute_pass.set_bind_group(0, &self.calculate_ghost_velocities_bind_group, &[]);
            compute_pass.set_pipeline(&self.calculate_ghost_velocities_pipeline);
            compute_pass.dispatch_workgroups((self.num_ghost_bodies / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Calculate Ghost Positions"),
            });
            compute_pass.set_bind_group(0, &self.calculate_ghost_positions_bind_group, &[]);
            compute_pass.set_pipeline(&self.calculate_ghost_positions_pipeline);
            compute_pass.dispatch_workgroups((self.num_ghost_bodies / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Render Ghost Positions"),
            });
            compute_pass.set_bind_group(0, &self.render_ghost_positions_bind_group, &[]);
            compute_pass.set_pipeline(&self.render_ghost_positions_pipeline);
            compute_pass.dispatch_workgroups((self.num_ghost_bodies / 64) as u32, 1, 1);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Render Massive Positions"),
            });
            compute_pass.set_bind_group(0, &self.render_massive_positions_bind_group, &[]);
            compute_pass.set_pipeline(&self.render_massive_positions_pipeline);
            compute_pass.dispatch_workgroups(self.num_massive_bodies as u32, 1, 1);
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.textures[0],
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.cpu_texture_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(
                        self.texture_extent.width * std::mem::size_of::<[f32; 4]>() as u32,
                    ),
                    rows_per_image: Some(self.texture_extent.height),
                },
            },
            self.texture_extent,
        );

        self.hardware.queue().submit(Some(encoder.finish()));

        let buffer_slice = self.cpu_texture_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        self.hardware.device().poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        if let Some(Ok(())) = receiver.receive().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();

            // Since contents are received in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // Unmaps buffer from memory, allowing it to be used again.
            drop(data);
            self.cpu_texture_buffer.unmap();

            // Update the image
            image.from_slice(&result);
        } else {
            panic!("failed to run compute on gpu!")
        }
    }
}

fn create_settings_buffer(hardware: &Hardware, init_settings: &[f32; 4]) -> wgpu::Buffer {
    hardware
        .device()
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("N-Body - Settings Buffer"),
            contents: bytemuck::cast_slice(init_settings),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
}

fn create_4d_buffer(hardware: &Hardware, data: &[[f32; 4]]) -> wgpu::Buffer {
    let data = data.iter().flatten().copied().collect::<Vec<_>>();
    hardware
        .device()
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("N-Body - 4D Buffer"),
            contents: bytemuck::cast_slice(data.as_slice()),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        })
}

fn create_texture_extent(nrows: u32, ncols: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        height: nrows,
        width: ncols,
        depth_or_array_layers: 1,
    }
}

fn create_texture(hardware: &Hardware, texture_extent: &wgpu::Extent3d) -> wgpu::Texture {
    hardware.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("N-Body - ReadWrite - Texture"),
        size: *texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[wgpu::TextureFormat::Rgba32Float],
    })
}

fn create_cpu_texture_buffer(hardware: &Hardware, texture_extent: &wgpu::Extent3d) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - CPU - Texture"),
        size: (texture_extent.width
            * texture_extent.height
            * std::mem::size_of::<[f32; 4]>() as u32) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_calculate_ghost_forces_pipeline_and_bind_group(
    hardware: &Hardware,
    settings_buffer: &wgpu::Buffer,
    ghost_positions: &wgpu::Buffer,
    massive_positions: &wgpu::Buffer,
    ghost_forces: &wgpu::Buffer,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("calculate_ghost_forces.wgsl");

    let bind_group_layout = create_calculate_ghost_forces_bind_group_layout(hardware.device());
    let shader_module =
        create_calculate_ghost_forces_shader_module(hardware.device(), shader_source);
    let pipeline_layout =
        create_calculate_ghost_forces_pipeline_layout(hardware.device(), &bind_group_layout);
    let pipeline =
        create_calculate_ghost_forces_pipeline(hardware.device(), &pipeline_layout, &shader_module);
    let bind_group = create_calculate_ghost_forces_bind_group(
        hardware.device(),
        &pipeline,
        settings_buffer,
        &ghost_positions,
        &massive_positions,
        ghost_forces,
    );

    (pipeline, bind_group)
}

fn create_calculate_ghost_velocities_pipeline_and_bind_group(
    hardware: &Hardware,
    settings_buffer: &wgpu::Buffer,
    ghost_velocities: &wgpu::Buffer,
    ghost_forces: &wgpu::Buffer,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("calculate_ghost_velocities.wgsl");

    let bind_group_layout = create_calculate_ghost_velocities_bind_group_layout(hardware.device());
    let shader_module =
        create_calculate_ghost_velocities_shader_module(hardware.device(), shader_source);
    let pipeline_layout =
        create_calculate_ghost_velocities_pipeline_layout(hardware.device(), &bind_group_layout);
    let pipeline = create_calculate_ghost_velocities_pipeline(
        hardware.device(),
        &pipeline_layout,
        &shader_module,
    );
    let bind_group = create_calculate_ghost_velocities_bind_group(
        hardware.device(),
        &pipeline,
        settings_buffer,
        ghost_forces,
        &ghost_velocities,
    );

    (pipeline, bind_group)
}

fn create_calculate_ghost_positions_pipeline_and_bind_group(
    hardware: &Hardware,
    settings_buffer: &wgpu::Buffer,
    ghost_positions: &wgpu::Buffer,
    ghost_velocities: &wgpu::Buffer,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("calculate_ghost_positions.wgsl");

    let bind_group_layout = create_calculate_ghost_positions_bind_group_layout(hardware.device());
    let shader_module =
        create_calculate_ghost_positions_shader_module(hardware.device(), shader_source);
    let pipeline_layout =
        create_calculate_ghost_positions_pipeline_layout(hardware.device(), &bind_group_layout);
    let pipeline = create_calculate_ghost_positions_pipeline(
        hardware.device(),
        &pipeline_layout,
        &shader_module,
    );
    let bind_group = create_calculate_ghost_positions_bind_group(
        hardware.device(),
        &pipeline,
        settings_buffer,
        &ghost_velocities,
        &ghost_positions,
    );

    (pipeline, bind_group)
}

fn create_render_ghost_positions_pipeline_and_bind_group(
    hardware: &Hardware,
    settings_buffer: &wgpu::Buffer,
    ghost_positions: &wgpu::Buffer,
    textures: &[wgpu::Texture; 2],
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("render_ghost_positions.wgsl");

    let bind_group_layout = create_render_ghost_positions_bind_group_layout(hardware.device());
    let shader_module =
        create_render_ghost_positions_shader_module(hardware.device(), shader_source);
    let pipeline_layout =
        create_render_ghost_positions_pipeline_layout(hardware.device(), &bind_group_layout);
    let pipeline =
        create_render_ghost_positions_pipeline(hardware.device(), &pipeline_layout, &shader_module);
    let bind_group = create_render_ghost_particles_bind_group(
        hardware.device(),
        &pipeline,
        settings_buffer,
        &ghost_positions,
        &textures[0],
    );

    (pipeline, bind_group)
}

fn create_render_massive_positions_pipeline_and_bind_group(
    hardware: &Hardware,
    settings_buffer: &wgpu::Buffer,
    massive_positions: &wgpu::Buffer,
    textures: &[wgpu::Texture; 2],
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let shader_source = include_str!("render_massive_positions.wgsl");

    let bind_group_layout = create_render_massive_positions_bind_group_layout(hardware.device());
    let shader_module =
        create_render_massive_positions_shader_module(hardware.device(), shader_source);
    let pipeline_layout =
        create_render_massive_positions_pipeline_layout(hardware.device(), &bind_group_layout);
    let pipeline = create_render_massive_positions_pipeline(
        hardware.device(),
        &pipeline_layout,
        &shader_module,
    );
    let bind_group = create_render_massive_particles_bind_group(
        hardware.device(),
        &pipeline,
        settings_buffer,
        &massive_positions,
        &textures[0],
    );

    (pipeline, bind_group)
}

fn create_calculate_ghost_forces_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Forces - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
        ],
    })
}

fn create_calculate_ghost_velocities_bind_group_layout(
    device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Velocities - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
        ],
    })
}

fn create_calculate_ghost_positions_bind_group_layout(
    device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Positions - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
        ],
    })
}

fn create_render_ghost_positions_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Render Ghost Positions - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba32Float,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
        ],
    })
}

fn create_render_massive_positions_bind_group_layout(
    device: &wgpu::Device,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("N-Body - Render Massive Positions - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    min_binding_size: None,
                    has_dynamic_offset: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba32Float,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
        ],
    })
}

fn create_calculate_ghost_forces_shader_module(
    device: &wgpu::Device,
    shader_source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Ghost Forces - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

fn create_calculate_ghost_velocities_shader_module(
    device: &wgpu::Device,
    shader_source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Ghost Velocities - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

fn create_calculate_ghost_positions_shader_module(
    device: &wgpu::Device,
    shader_source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Ghost Positions - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

fn create_render_ghost_positions_shader_module(
    device: &wgpu::Device,
    shader_source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Ghost Positions - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

fn create_render_massive_positions_shader_module(
    device: &wgpu::Device,
    shader_source: &str,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("N-Body - Render Massive Positions - Shader Module"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

fn create_calculate_ghost_forces_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Forces - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_calculate_ghost_velocities_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Velocities - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_calculate_ghost_positions_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Calculate Ghost Positions - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_render_ghost_positions_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Render Ghost Positions - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_render_massive_positions_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("N-Body - Render Massive Positions - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_calculate_ghost_forces_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Calculate Ghost Forces - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_calculate_ghost_velocities_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Calculate Ghost Velocities - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_calculate_ghost_positions_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Calculate Ghost Positions - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_render_ghost_positions_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Render Ghost Positions - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_render_massive_positions_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("N-Body - Render Massive Positions - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_calculate_ghost_forces_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    settings_buffer: &wgpu::Buffer,
    ghost_positions: &wgpu::Buffer,
    massive_positions: &wgpu::Buffer,
    ghost_forces: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Calculate Ghost Forces - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: settings_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: ghost_positions.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: massive_positions.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: ghost_forces.as_entire_binding(),
            },
        ],
    })
}

fn create_calculate_ghost_velocities_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    settings_buffer: &wgpu::Buffer,
    ghost_forces: &wgpu::Buffer,
    ghost_velocities: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Calculate Ghost Velocities - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: settings_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: ghost_forces.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: ghost_velocities.as_entire_binding(),
            },
        ],
    })
}

fn create_calculate_ghost_positions_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    settings_buffer: &wgpu::Buffer,
    ghost_velocities: &wgpu::Buffer,
    ghost_positions: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Calculate Ghost Velocities - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: settings_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: ghost_velocities.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: ghost_positions.as_entire_binding(),
            },
        ],
    })
}

fn create_render_ghost_particles_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    settings_buffer: &wgpu::Buffer,
    massive_positions: &wgpu::Buffer,
    texture: &wgpu::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Render Ghost Positions - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: settings_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: massive_positions.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(
                    &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
        ],
    })
}

fn create_render_massive_particles_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    settings_buffer: &wgpu::Buffer,
    massive_positions: &wgpu::Buffer,
    texture: &wgpu::Texture,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("N-Body - Render Massive Positions - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: settings_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: massive_positions.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(
                    &texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
        ],
    })
}
