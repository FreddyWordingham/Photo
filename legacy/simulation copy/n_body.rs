use rand::Rng;
use wgpu::util::DeviceExt;

use crate::{hardware, Hardware};

pub struct NBody {
    positions: Vec<[f32; 2]>,
    velocities: Vec<[f32; 2]>,
    masses: Vec<f32>,
    forces: Vec<[f32; 2]>,

    hardware: Hardware,
    uniform_buffer: wgpu::Buffer,
    positions_buffer: wgpu::Buffer,
    velocities_buffer: wgpu::Buffer,
    masses_buffer: wgpu::Buffer,
    forces_buffer: wgpu::Buffer,
    read_buffer: wgpu::Buffer,
    calculate_forces_pipeline: wgpu::ComputePipeline,
    calculate_forces_bind_group: wgpu::BindGroup,
}

impl NBody {
    pub async fn new<R: Rng>(rng: &mut R, num_bodies: usize, grav_force: f32, speed: f32) -> Self {
        debug_assert!(num_bodies > 0);

        let (positions_a, velocities_a, masses_a, forces_a) =
            Self::orbit_circle(rng, num_bodies / 2, speed, [-2.0, -3.0], [0.0, 0.1]);
        let (positions_b, velocities_b, masses_b, forces_b) =
            Self::orbit_circle(rng, num_bodies / 2, speed, [2.0, 3.0], [0.0, -0.1]);
        let mut positions = [positions_a, positions_b].concat();
        let mut velocities = [velocities_a, velocities_b].concat();
        let mut masses = [masses_a, masses_b].concat();
        let mut forces = [forces_a, forces_b].concat();

        positions[1] = positions[num_bodies / 2];
        velocities[1] = velocities[num_bodies / 2];
        masses[1] = masses[num_bodies / 2];
        forces[1] = forces[num_bodies / 2];

        let hardware = hardware::Hardware::new().await;

        let uniform_data = [num_bodies as f32, grav_force];

        let uniform_buffer = create_uniform_buffer(&hardware, &uniform_data);
        let positions_buffer = create_positions_buffer::<[f32; 2]>(&hardware, num_bodies);
        let velocities_buffer = create_velocities_buffer::<[f32; 2]>(&hardware, num_bodies);
        let masses_buffer = create_masses_buffer::<f32>(&hardware, num_bodies);
        let forces_buffer = create_forces_buffer::<[f32; 2]>(&hardware, num_bodies);
        let read_buffer = create_read_buffer::<[f32; 2]>(&hardware, num_bodies);

        let (calculate_forces_pipeline, calculate_forces_bind_group) =
            create_calculate_forces_pipeline_and_bind_group(
                &hardware,
                &uniform_buffer,
                &positions_buffer,
                &velocities_buffer,
                &masses_buffer,
                &forces_buffer,
            );

        Self {
            positions,
            velocities,
            masses,
            forces,

            hardware,
            uniform_buffer,
            positions_buffer,
            velocities_buffer,
            masses_buffer,
            forces_buffer,
            read_buffer,

            calculate_forces_pipeline,
            calculate_forces_bind_group,
        }
    }

    pub async fn run(&mut self, grav_force: f32) {
        self.update_uniform_buffer(grav_force);
        self.update_positions_buffer();
        self.update_velocities_buffer();
        self.update_masses_buffer();

        let mut encoder =
            self.hardware
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("N-Body"),
                });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("N-Body - Calculate Forces"),
            });
            compute_pass.set_bind_group(0, &self.calculate_forces_bind_group, &[]);
            compute_pass.set_pipeline(&self.calculate_forces_pipeline);

            compute_pass.dispatch_workgroups(
                (self.positions.len() as f32 / 64.0).ceil() as u32,
                1,
                1,
            );
        }

        encoder.copy_buffer_to_buffer(
            &self.forces_buffer,
            0,
            &self.read_buffer,
            0,
            (self.positions.len() * std::mem::size_of::<[f32; 2]>()) as u64,
        );

        self.hardware.queue().submit(Some(encoder.finish()));

        let forces_buffer_slice = self.read_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        forces_buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        self.hardware.device().poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        if let Some(Ok(())) = receiver.receive().await {
            // Gets contents of buffer
            let data = forces_buffer_slice.get_mapped_range();

            // Since contents are received in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // Unmaps buffer from memory, allowing it to be used again.
            drop(data);
            self.read_buffer.unmap();

            self.forces = result;
        } else {
            panic!("failed to run compute on gpu!")
        }
    }

    fn update_uniform_buffer(&self, grav_force: f32) {
        let uniform_data = [self.positions.len() as f32, grav_force];
        self.hardware.queue().write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&uniform_data),
        );
    }

    fn update_positions_buffer(&self) {
        self.hardware.queue().write_buffer(
            &self.positions_buffer,
            0,
            bytemuck::cast_slice(&self.positions),
        );
    }

    fn update_velocities_buffer(&self) {
        self.hardware.queue().write_buffer(
            &self.velocities_buffer,
            0,
            bytemuck::cast_slice(&self.velocities),
        );
    }

    fn update_masses_buffer(&self) {
        self.hardware.queue().write_buffer(
            &self.masses_buffer,
            0,
            bytemuck::cast_slice(&self.masses),
        );
    }

    fn orbit_circle<R: Rng>(
        rng: &mut R,
        num_bodies: usize,
        speed: f32,
        offset: [f32; 2],
        velocity_offset: [f32; 2],
    ) -> (Vec<[f32; 2]>, Vec<[f32; 2]>, Vec<f32>, Vec<[f32; 2]>) {
        let mut positions = Vec::with_capacity(num_bodies);
        let mut velocities = Vec::with_capacity(num_bodies);
        let mut masses = Vec::with_capacity(num_bodies);
        let forces = vec![[0.0, 0.0]; num_bodies];
        for _ in 0..num_bodies {
            let mut px = rng.gen_range(-1.0f32..1.0);
            let mut py = rng.gen_range(-1.0..1.0);
            let mut r = px * px + py * py;

            while r >= 1.0 || r < 0.5 {
                px = rng.gen_range(-1.0..1.0);
                py = rng.gen_range(-1.0..1.0);
                r = px * px + py * py;
            }

            let theta = px.atan2(py);
            let vx = speed * theta.cos();
            let vy = -speed * theta.sin();

            positions.push([px + offset[0], py + offset[1]]);
            velocities.push([vx + velocity_offset[0], vy + velocity_offset[1]]);
            masses.push(1.0e-3 / num_bodies as f32);
        }

        positions[0] = offset;
        velocities[0] = velocity_offset;
        masses[0] = 1000.0;

        // positions[1] = [2.0, 0.5];
        // velocities[1] = [-0.2, 0.0];
        // masses[1] = 100.0;

        (positions, velocities, masses, forces)
    }

    pub fn positions(&self) -> &[[f32; 2]] {
        &self.positions
    }

    pub fn velocities(&self) -> &[[f32; 2]] {
        &self.velocities
    }

    pub fn masses(&self) -> &[f32] {
        &self.masses
    }

    fn calculate_forces(&self) -> Vec<[f32; 2]> {
        let mut forces = vec![[0.0, 0.0]; self.positions.len()];

        for i in 0..self.positions.len() {
            let (head, tail) = self.positions.split_at(i + 1);
            let pi = &head[i];
            let mi = self.masses[i];

            for (j, pj) in tail.iter().enumerate() {
                let j = i + 1 + j;
                let mj = self.masses[j];

                let dx = pj[0] - pi[0];
                let dy = pj[1] - pi[1];
                let r2 = dx * dx + dy * dy;
                let r = r2.sqrt();
                let f_over_r = mi * mj / (r * r2);
                let fx = f_over_r * dx;
                let fy = f_over_r * dy;

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[j][0] -= fx;
                forces[j][1] -= fy;
            }
        }

        forces
    }

    fn update_velocities(&mut self, dt: f32) {
        for ([vx, vy], ([fx, fy], m)) in self
            .velocities
            .iter_mut()
            .zip(self.forces.iter().zip(&self.masses))
        {
            *vx += fx * dt / m;
            *vy += fy * dt / m;
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for ([px, py], [vx, vy]) in self.positions.iter_mut().zip(self.velocities.iter()) {
            *px += vx * dt;
            *py += vy * dt;
        }
    }

    pub async fn step(&mut self, dt: f32, grav_force: f32) {
        self.run(grav_force).await;
        self.update_velocities(dt);
        self.update_positions(dt);
    }
}

fn create_uniform_buffer(hardware: &Hardware, uniform_data: &[f32; 2]) -> wgpu::Buffer {
    hardware
        .device()
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("N-Body - Uniform Buffer"),
            contents: bytemuck::cast_slice(uniform_data),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
}

fn create_positions_buffer<T>(hardware: &Hardware, num_bodies: usize) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - Positions Buffer"),
        size: (num_bodies * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_velocities_buffer<T>(hardware: &Hardware, num_bodies: usize) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - Velocities Buffer"),
        size: (num_bodies * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_masses_buffer<T>(hardware: &Hardware, num_bodies: usize) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - Masses Buffer"),
        size: (num_bodies * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_forces_buffer<T>(hardware: &Hardware, num_bodies: usize) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - Forces Buffer"),
        size: (num_bodies * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

fn create_read_buffer<T>(hardware: &Hardware, num_bodies: usize) -> wgpu::Buffer {
    hardware.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("N-Body - Read Buffer"),
        size: (num_bodies * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_calculate_forces_pipeline_and_bind_group(
    hardware: &Hardware,
    uniform_buffer: &wgpu::Buffer,
    positions_buffer: &wgpu::Buffer,
    velocities_buffer: &wgpu::Buffer,
    masses_buffer: &wgpu::Buffer,
    forces_buffer: &wgpu::Buffer,
) -> (wgpu::ComputePipeline, wgpu::BindGroup) {
    let device = hardware.device();

    let shader_source = include_str!("calculate_forces.wgsl");

    let bind_group_layout = create_calculate_forces_bind_group_layout(device);
    let shader_module = create_calculate_forces_shader_module(shader_source, device);
    let pipeline_layout = create_calculate_forces_pipeline_layout(device, &bind_group_layout);
    let calculate_forces_pipeline =
        create_calculate_forces_pipeline(device, &pipeline_layout, &shader_module);
    let calculate_forces_bind_group = create_calculate_forces_bind_group(
        device,
        &calculate_forces_pipeline,
        uniform_buffer,
        positions_buffer,
        velocities_buffer,
        masses_buffer,
        forces_buffer,
    );

    (calculate_forces_pipeline, calculate_forces_bind_group)
}

fn create_calculate_forces_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Calculate Forces - Bind Group Layout"),
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
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    })
}

fn create_calculate_forces_shader_module(
    shader_source: &str,
    device: &wgpu::Device,
) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Invert - Compute Shader Module"),
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&shader_source)),
    })
}

fn create_calculate_forces_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Calculate Forces - Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    })
}

fn create_calculate_forces_pipeline(
    device: &wgpu::Device,
    pipeline_layout: &wgpu::PipelineLayout,
    shader_module: &wgpu::ShaderModule,
) -> wgpu::ComputePipeline {
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Calculate Forces - Pipeline"),
        layout: Some(pipeline_layout),
        module: shader_module,
        entry_point: "main",
    })
}

fn create_calculate_forces_bind_group(
    device: &wgpu::Device,
    pipeline: &wgpu::ComputePipeline,
    uniform_buffer: &wgpu::Buffer,
    positions_buffer: &wgpu::Buffer,
    velocities_buffer: &wgpu::Buffer,
    masses_buffer: &wgpu::Buffer,
    forces_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Calculate Forces - Bind Group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: positions_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: velocities_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: masses_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: forces_buffer.as_entire_binding(),
            },
        ],
    })
}
