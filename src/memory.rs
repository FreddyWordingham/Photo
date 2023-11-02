use wgpu::util::DeviceExt;

use crate::{
    geometry::{BVHBuilder, Scene},
    Camera, Settings,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
];

const INDICES: &[u16] = &[
    0, 1, 3, //
    1, 2, 3, //
];

pub struct Memory {
    // Drawing
    pub num_indices: u32,

    // Uniforms
    pub settings_uniform: wgpu::Buffer,
    pub camera_uniform: wgpu::Buffer,

    // Textures
    pub display_texture: wgpu::Texture,
    pub offscreen_texture: wgpu::Texture,
    pub display_view: wgpu::TextureView,
    pub offscreen_view: wgpu::TextureView,
    pub display_sampler: wgpu::Sampler,

    // Scene
    pub scene_positions_buffer: wgpu::Buffer,
    pub scene_position_indices_buffer: wgpu::Buffer,
    pub scene_normals_buffer: wgpu::Buffer,
    pub scene_normal_indices_buffer: wgpu::Buffer,

    // BVH
    pub bvh_data: wgpu::Buffer,
    pub bvh_indices: wgpu::Buffer,

    // Rendering
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl<'a> Memory {
    pub fn new(
        resolution: [u32; 2],
        settings: &Settings,
        camera: &Camera,
        scene: Scene,
        device: &wgpu::Device,
    ) -> Self {
        let [width, height] = resolution;

        let settings_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Settings Uniform"),
            contents: bytemuck::cast_slice(&settings.as_buffer()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform"),
            contents: bytemuck::cast_slice(&camera.as_buffer()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Display texture
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        let display_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Display Texture"),
            view_formats: &[],
        });
        let offscreen_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Secondary Texture"),
            view_formats: &[],
        });
        let display_view = display_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let offscreen_view = offscreen_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let display_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            // mag_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Scene data
        let scene_positions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Positions Buffer"),
            contents: bytemuck::cast_slice(&scene.positions_buffer()),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let scene_position_indices_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene Position Indices Buffer"),
                contents: bytemuck::cast_slice(&scene.position_indices_buffer()),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let scene_normals_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Scene Normals Buffer"),
            contents: bytemuck::cast_slice(&scene.normals_buffer()),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let scene_normal_indices_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scene Normal Indices Buffer"),
                contents: bytemuck::cast_slice(&scene.normal_indices_buffer()),
                usage: wgpu::BufferUsages::STORAGE,
            });

        // BVH data
        let mut bvh = BVHBuilder::new(&scene);
        bvh.build();
        let bvh_data = bvh.bvh_data();
        let bvh_indices = bvh.bvh_indices();

        let bvh_data = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("BVH Data"),
            contents: bytemuck::cast_slice(&bvh_data),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let bvh_indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("BVH Indices"),
            contents: bytemuck::cast_slice(&bvh_indices),
            usage: wgpu::BufferUsages::STORAGE,
        });

        // Rendering data
        let num_indices = INDICES.len() as u32;
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            num_indices,
            settings_uniform,
            camera_uniform,
            display_texture,
            offscreen_texture,
            display_view,
            offscreen_view,
            display_sampler,
            scene_positions_buffer,
            scene_position_indices_buffer,
            scene_normals_buffer,
            scene_normal_indices_buffer,
            bvh_data,
            bvh_indices,
            vertex_buffer,
            index_buffer,
        }
    }
}
