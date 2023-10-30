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

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.hardware.config.width = new_size.width;
            self.hardware.config.height = new_size.height;
            self.hardware
                .surface
                .configure(&self.hardware.device, &self.hardware.config);
        }
    }

    pub fn update(&self) {
        println!("Updating!");
    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        self.hardware.queue.write_buffer(
            &self.memory.camera_uniform,
            0,
            bytemuck::cast_slice(&self.camera.as_slice()),
        );

        let output = self.hardware.surface.get_current_texture()?;

        let screen_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.hardware
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Texture"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.memory.display_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Draw Background"),
                timestamp_writes: None,
            });
            compute_pass.set_bind_group(0, &self.pipelines.draw_background_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.draw_background_pipeline);
            compute_pass.dispatch_workgroups(
                self.settings.resolution[0],
                self.settings.resolution[1],
                1,
            );
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Draw Scene"),
                timestamp_writes: None,
            });
            compute_pass.set_bind_group(0, &self.pipelines.draw_scene_bind_group, &[]);
            compute_pass.set_pipeline(&self.pipelines.draw_scene_pipeline);
            compute_pass.dispatch_workgroups(
                self.settings.resolution[0],
                self.settings.resolution[1],
                1,
            );
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &screen_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipelines.display_pipeline);
            render_pass.set_bind_group(0, &self.pipelines.display_bind_group, &[]);
            render_pass.set_index_buffer(
                self.memory.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.set_vertex_buffer(0, self.memory.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..self.memory.num_indices, 0, 0..1);
        }

        self.hardware
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
