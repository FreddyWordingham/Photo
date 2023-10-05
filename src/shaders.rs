use wgpu;
use wgpu::util::DeviceExt;

use crate::{Chunk, Hardware};

pub struct Shaders<'a> {
    hardware: &'a Hardware,
    buffer_size: usize,
    storage_buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer,
    compute_pipelines: Vec<wgpu::ComputePipeline>,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl<'a> Shaders<'a> {
    pub fn new(hardware: &'a Hardware, chunks: &[Chunk], shader_codes: Vec<&str>) -> Self {
        let buffer_size = std::mem::size_of_val(chunks);
        if buffer_size % 16 != 0 {
            panic!("Buffer size must be a multiple of 16 bytes");
        }

        // Instantiates buffer without data.
        let staging_buffer = hardware.device().create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size as u64,
            usage: wgpu::BufferUsages::MAP_READ |   // `MAP_READ` allows it to be read (outside the shader).
                    wgpu::BufferUsages::COPY_DST, // `COPY_DST` allows it to be the destination of the copy.
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`cols`).
        let storage_buffer =
            hardware
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Storage Buffer"),
                    contents: bytemuck::cast_slice(chunks),
                    usage: wgpu::BufferUsages::STORAGE // `STORAGE` can be used with a bind group, and thus available to a shader.
                        | wgpu::BufferUsages::COPY_DST // `COPY_DST` allows it to be the destination of a copy.
                        | wgpu::BufferUsages::COPY_SRC, // `COPY_SRC` allows it to be the source of a copy.
                });

        // Load the shaders
        let shader_modules = shader_codes
            .iter()
            .map(|s| {
                hardware
                    .device()
                    .create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(s)),
                    })
            })
            .collect::<Vec<_>>();

        // Instantiates the pipeline.
        let compute_pipelines: Vec<_> = shader_modules
            .iter()
            .map(|module| {
                hardware
                    .device()
                    .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                        label: None,
                        layout: None,
                        module: module,
                        entry_point: "main",
                    })
            })
            .collect();

        // Instantiates the bind groups, once again specifying the binding of buffers.
        let bind_groups = compute_pipelines
            .iter()
            .map(|compute_pipeline| {
                let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
                hardware
                    .device()
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: storage_buffer.as_entire_binding(),
                        }],
                    })
            })
            .collect();

        Self {
            hardware,
            buffer_size,
            staging_buffer,
            storage_buffer,
            compute_pipelines,
            bind_groups,
        }
    }

    pub async fn run(&self, chunks: &[Chunk], shader_index: usize) -> Vec<Chunk> {
        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self
            .hardware
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_bind_group(0, &self.bind_groups[shader_index], &[]);
            cpass.set_pipeline(&self.compute_pipelines[shader_index]);
            cpass.dispatch_workgroups(chunks.len() as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_buffer_to_buffer(
            &self.storage_buffer,
            0,
            &self.staging_buffer,
            0,
            self.buffer_size as u64,
        );

        // Submits command encoder for processing
        self.hardware.queue().submit(Some(encoder.finish()));

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let buffer_slice = self.staging_buffer.slice(..);
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
            self.staging_buffer.unmap();

            result
        } else {
            panic!("failed to run compute on gpu!")
        }
    }
}
