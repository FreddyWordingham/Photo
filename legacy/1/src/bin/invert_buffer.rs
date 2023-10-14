// use photo::Image;

// fn main() {
//     println!("Hello, world!");
//     let mut image = Image::load("input/image.png");

//     let mut gpu = pollster::block_on(GPUProcessor::new());
//     pollster::block_on(gpu.run_shader(&mut image));

//     image.save("output/image.png");
// }

// use wgpu::{self, util::DeviceExt};
// struct GPUProcessor {
//     device: wgpu::Device,
//     queue: wgpu::Queue,
// }

// impl GPUProcessor {
//     pub async fn new() -> Self {
//         // Instantiates instance of WebGPU
//         let instance = wgpu::Instance::default();

//         // `request_adapter` instantiates the general connection to the GPU
//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions::default())
//             .await
//             .expect("No suitable GPU adapters found on the system");

//         // `request_device` instantiates the feature specific connection to the GPU.
//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     label: None,
//                     features: wgpu::Features::empty(),
//                     limits: wgpu::Limits::downlevel_defaults(),
//                 },
//                 None,
//             )
//             .await
//             .expect("Failed to create device and queue");

//         Self { device, queue }
//     }

//     /// Run a compute shader on the given image.
//     pub async fn run_shader(&self, image: &mut Image) {
//         println!("Running invert.wgsl");
//         println!("Loading shader...");

//         let shader_source = include_str!("invert_buffer.wgsl");
//         let shader_module = self
//             .device
//             .create_shader_module(wgpu::ShaderModuleDescriptor {
//                 label: Some("Invert - Compute Shader Module"),
//                 source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader_source)),
//             });

//         println!("Initialising buffers...");
//         let uniform_data = [0.0f32; 4];
//         let uniform_buffer = self
//             .device
//             .create_buffer_init(&wgpu::util::BufferInitDescriptor {
//                 label: Some("Invert - Uniform Buffer"),
//                 contents: bytemuck::cast_slice(&uniform_data),
//                 usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//             });

//         let storage_buffer_size = image.width() * image.height() * 4 * 4;
//         let storage_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("Invert - Storage Buffer"),
//             size: storage_buffer_size as u64,
//             usage: wgpu::BufferUsages::STORAGE
//                 | wgpu::BufferUsages::COPY_DST
//                 | wgpu::BufferUsages::COPY_SRC,
//             mapped_at_creation: false,
//         });

//         let copy_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
//             label: Some("Invert - Copy Buffer"),
//             size: storage_buffer_size as u64,
//             usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
//             mapped_at_creation: false,
//         });

//         let bind_group_layout =
//             self.device
//                 .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//                     label: Some("Invert - Bind Group Layout"),
//                     entries: &[
//                         wgpu::BindGroupLayoutEntry {
//                             binding: 0,
//                             visibility: wgpu::ShaderStages::COMPUTE,
//                             ty: wgpu::BindingType::Buffer {
//                                 min_binding_size: None,
//                                 has_dynamic_offset: false,
//                                 ty: wgpu::BufferBindingType::Uniform,
//                             },
//                             count: None,
//                         },
//                         wgpu::BindGroupLayoutEntry {
//                             binding: 1,
//                             visibility: wgpu::ShaderStages::COMPUTE,
//                             ty: wgpu::BindingType::Buffer {
//                                 min_binding_size: None,
//                                 has_dynamic_offset: false,
//                                 ty: wgpu::BufferBindingType::Storage { read_only: false },
//                             },
//                             count: None,
//                         },
//                     ],
//                 });

//         let pipeline_layout = self
//             .device
//             .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//                 label: Some("Invert - Compute Pipeline Layout"),
//                 bind_group_layouts: &[&bind_group_layout],
//                 push_constant_ranges: &[],
//             });

//         let pipeline = self
//             .device
//             .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
//                 label: Some("Invert - Compute Pipeline"),
//                 layout: Some(&pipeline_layout),
//                 module: &shader_module,
//                 entry_point: "main",
//             });

//         let bind_group_layout = pipeline.get_bind_group_layout(0);
//         let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
//             label: Some("Invert - Bind Group"),
//             layout: &bind_group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: uniform_buffer.as_entire_binding(),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: storage_buffer.as_entire_binding(),
//                 },
//             ],
//         });

//         println!("Write to GPU...");
//         self.queue.write_buffer(
//             &storage_buffer,
//             0,
//             bytemuck::cast_slice(image.as_1d_f32().as_slice()),
//         );
//         self.queue
//             .write_buffer(&uniform_buffer, 0, bytemuck::bytes_of(&uniform_data));

//         println!("Encoding shader...");
//         let mut encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                 label: Some("Invert - Command Encoder"),
//             });

//         {
//             let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//                 label: Some("Invert - Compute Pass"),
//             });
//             cpass.set_bind_group(0, &bind_group, &[]);
//             cpass.set_pipeline(&pipeline);
//             cpass.dispatch_workgroups(
//                 ((image.width() * image.height()) / 64).try_into().unwrap(),
//                 1,
//                 1,
//             );
//         }

//         println!("Copy data to CPU...");
//         encoder.copy_buffer_to_buffer(
//             &storage_buffer,
//             0,
//             &copy_buffer,
//             0,
//             storage_buffer_size as u64,
//         );

//         println!("Submit...");
//         self.queue.submit(Some(encoder.finish()));

//         println!("Read from GPU...");
//         let buffer_slice = copy_buffer.slice(..);
//         let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
//         buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

//         // Poll the device in a blocking manner so that our future resolves.
//         // In an actual application, `device.poll(...)` should
//         // be called in an event loop or on another thread.
//         self.device.poll(wgpu::Maintain::Wait);

//         // Awaits until `buffer_future` can be read from
//         if let Some(Ok(())) = receiver.receive().await {
//             // Gets contents of buffer
//             let data = buffer_slice.get_mapped_range();

//             // Since contents are received in bytes, this converts these bytes back to u32
//             let result = bytemuck::cast_slice(&data).to_vec();

//             // Unmaps buffer from memory, allowing it to be used again.
//             drop(data);
//             copy_buffer.unmap();

//             image.from_1d_f32(&result);
//         } else {
//             panic!("failed to run compute on gpu!")
//         }
//     }
// }

fn main() {}
