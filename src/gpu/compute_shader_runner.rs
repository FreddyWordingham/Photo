use bytemuck::Pod;
use wgpu;
use wgpu::util::DeviceExt;

use crate::gpu::Hardware;

// Run compute shaders which take in a single uniform object and and operate on an array of read-write storage.
pub struct ComputeShaderRunner<'a> {
    hardware: &'a Hardware,
    buffer_size: usize,
    uniform_buffer: wgpu::Buffer,
    storage_buffer: wgpu::Buffer,
    copy_buffer: wgpu::Buffer,
    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
}

impl<'a> ComputeShaderRunner<'a> {
    pub fn new<BufferData: Pod, UniformData: Pod>(
        hardware: &'a Hardware,
        shader_code: &str,
        uniform: UniformData,
        buffer_data: &[BufferData],
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
            hardware,
            buffer_size,
            uniform_buffer,
            storage_buffer,
            copy_buffer,
            compute_pipeline,
            bind_group,
        }
    }

    pub async fn run<BufferData: Pod, UniformData: Pod>(
        &self,
        uniform: &UniformData,
        buffer_data: &[BufferData],
    ) -> Vec<BufferData> {
        self.write_uniform(uniform);
        let mut encoder = self.execute_pipeline(buffer_data);
        self.copy_data_to_cpu(&mut encoder);

        // Submits command encoder for processing
        self.hardware.queue().submit(Some(encoder.finish()));

        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let buffer_slice = self.copy_buffer.slice(..);
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
            self.copy_buffer.unmap();

            result
        } else {
            panic!("failed to run compute on gpu!")
        }
    }

    // Writes the uniform to the uniform buffer.
    pub fn write_uniform<Uniform: Pod>(&self, uniform: &Uniform) {
        self.hardware
            .queue()
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniform));
    }

    // Executes the pipelines.
    pub fn execute_pipeline<BufferData: Pod>(
        &self,
        read_write_storage: &[BufferData],
    ) -> wgpu::CommandEncoder {
        let mut encoder = self
            .hardware
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.dispatch_workgroups(read_write_storage.len() as u32, 1, 1);
        }
        encoder
    }

    // Copies data from storage buffer on GPU to copy buffer on CPU.
    pub fn copy_data_to_cpu(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.copy_buffer_to_buffer(
            &self.storage_buffer,
            0,
            &self.copy_buffer,
            0,
            self.buffer_size as u64,
        );
    }
}

// Helper function to validate buffer size
fn validate_buffer_size<BufferData: Pod>(read_write_storage: &[BufferData]) -> usize {
    let buffer_size = std::mem::size_of_val(read_write_storage);
    if buffer_size % 16 != 0 {
        panic!("Buffer size must be a multiple of 16 bytes");
    }
    buffer_size
}

// Helper function to create a buffer
fn create_buffer<BufferData: Pod>(
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
