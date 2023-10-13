use wgpu::{self, util::DeviceExt};

use crate::{Hardware, Image};

/// This compute shader can be used to mutate an image.
/// - [2; f32] uniform buffer
/// - [nrows * ncols; rgba32float] read texture
/// - [nrows * ncols; rgba32float] write texture
pub struct Shader {
    nrows: u32,
    ncols: u32,
    hardware: Hardware,
    texture_extent: wgpu::Extent3d,
    uniform_buffer: wgpu::Buffer,
    read_texture: wgpu::Texture,
    write_texture: wgpu::Texture,
    cpu_buffer: wgpu::Buffer,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

impl Shader {
    pub async fn new(shader_source: &str, nrows: u32, ncols: u32, hardware: Hardware) -> Shader {
        let uniform_data = [0.0f32; 2];
        let texture_extent = create_texture_extent(nrows, ncols);

        let uniform_buffer = create_uniform_buffer(&hardware, &uniform_data);
        let read_texture = create_read_texture(&hardware, &texture_extent);
        let write_texture = create_write_texture(&hardware, &texture_extent);
        let cpu_buffer = create_cpu_buffer(nrows, ncols, &hardware);

        let (pipeline, bind_group) = create_pipeline_and_bind_group(
            shader_source,
            &hardware,
            &uniform_buffer,
            &read_texture,
            &write_texture,
        );

        Self {
            nrows,
            ncols,
            hardware,
            texture_extent,
            uniform_buffer,
            read_texture,
            write_texture,
            cpu_buffer,
            pipeline,
            bind_group,
        }
    }

    pub fn write_image_to_gpu(&mut self, image: &Image) {
        self.hardware.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.read_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(image.as_slice()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.ncols * 4 * 4),
                rows_per_image: Some(self.nrows),
            },
            self.texture_extent,
        );
    }

    pub fn write_uniform_to_gpu(&mut self, uniform: &[f32; 2]) {
        self.hardware
            .queue()
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniform));
    }

    fn run_compute_pass(&mut self) -> wgpu::CommandEncoder {
        let mut encoder =
            self.hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Invert - Command Encoder"),
                });

        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Invert - Compute Pass"),
            });
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.set_pipeline(&self.pipeline);
            cpass.dispatch_workgroups(
                (self.ncols / 8).try_into().unwrap(),
                (self.nrows / 8).try_into().unwrap(),
                1,
            );
        }

        encoder
    }

    fn read_image_from_gpu(&mut self, encoder: &mut wgpu::CommandEncoder) {
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.write_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.cpu_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.ncols * 4 * 4),
                    rows_per_image: Some(self.nrows),
                },
            },
            self.texture_extent,
        );
    }

    /// Run a compute shader on the given image.
    pub async fn run_shader(&mut self, image: &mut Image) {
        self.run_compute_pass();
        let mut encoder = self.run_compute_pass();
        self.read_image_from_gpu(&mut encoder);
        self.hardware.queue().submit(Some(encoder.finish()));

        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        let buffer_slice = self.cpu_buffer.slice(..);
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
            self.cpu_buffer.unmap();

            image.from_slice(&result);
        } else {
            panic!("failed to run compute on gpu!")
        }
    }
}

fn create_texture_extent(nrows: u32, ncols: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        height: nrows,
        width: ncols,
        depth_or_array_layers: 1,
    }
}

fn create_uniform_buffer(hardware: &Hardware, uniform_data: &[f32; 2]) -> wgpu::Buffer {
    hardware
        .device()
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Invert - Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
}

fn create_read_texture(hardware: &Hardware, texture_extent: &wgpu::Extent3d) -> wgpu::Texture {
    hardware.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Invert - Read Texture"),
        size: *texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[wgpu::TextureFormat::Rgba32Float],
    })
}

fn create_write_texture(hardware: &Hardware, texture_extent: &wgpu::Extent3d) -> wgpu::Texture {
    hardware.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Invert - Write Texture"),
        size: *texture_extent,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[wgpu::TextureFormat::Rgba32Float],
    })
}

fn create_cpu_buffer(nrows: u32, ncols: u32, hardware: &Hardware) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("Invert - Copy Buffer"),
        size: (nrows * ncols * 4 * 4) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_pipeline_and_bind_group(
    shader_source: &str,
    hardware: &Hardware,
    uniform_buffer: &wgpu::Buffer,
    read_texture: &wgpu::Texture,
    write_texture: &wgpu::Texture,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let device = hardware.device();

    let bind_group_layout = create_bind_group_layout(device);
    let shader_module = create_shader_module(shader_source, device);
    let pipeline_layout = create_pipeline_layout(device, &bind_group_layout);
    let pipeline = create_pipeline(device, &pipeline_layout, &shader_module);
    let bind_group = create_bind_group(
        device,
        &pipeline,
        uniform_buffer,
        read_texture,
        write_texture,
    );

    (pipeline, bind_group)
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Invert - Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    min_binding_size: None,
                    has_dynamic_offset: false,
                    ty: wgpu::BufferBindingType::Uniform,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::ReadOnly,
                    format: wgpu::TextureFormat::Rgba32Float,
                    view_dimension: wgpu::TextureViewDimension::D2,
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

fn create_shader_module(shader_source: &str, device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Invert - Compute Shader Module"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_source)),
    })
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Invert - Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Invert - Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "main",
    })
}

fn create_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    uniform_buffer: &wgpu::Buffer,
    read_texture: &wgpu::Texture,
    write_texture: &wgpu::Texture,
) -> wgpu::BindGroup {
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Invert - Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(
                    &read_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(
                    &write_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                ),
            },
        ],
    })
}
