import ray_tracer_kernel from "./shaders/shadow_tracer.wgsl";
import display_shader from "./shaders/display.wgsl";

import { Scene } from "./scene";
import { position_and_target_to_spherical } from "./util";

export class Renderer {
    // Device/Context objects
    canvas: HTMLCanvasElement;
    format: GPUTextureFormat;
    adapter!: GPUAdapter;
    device!: GPUDevice;
    context: GPUCanvasContext;

    // Assets
    colour_buffer: GPUTexture;
    colour_buffer_view: GPUTextureView;
    sampler: GPUSampler;
    scene_buffer: GPUBuffer;
    sphere_buffer: GPUBuffer;
    node_buffer: GPUBuffer;
    sphere_indices_buffer: GPUBuffer;

    // Pipeline
    ray_tracer_pipeline: GPUComputePipeline;
    ray_tracer_bind_group: GPUBindGroup;
    display_pipeline: GPURenderPipeline;
    display_bind_group: GPUBindGroup;

    // Scene to render
    lambda: number = 0.0;
    scene: Scene;

    constructor(canvas: HTMLCanvasElement, scene: Scene) {
        this.canvas = canvas;
        this.scene = scene;
    }

    async init() {
        await this.setup_device();
        this.create_assets();
        this.make_pipeline();
    }

    async setup_device() {
        this.format = "bgra8unorm";
        this.adapter = <GPUAdapter>await navigator.gpu?.requestAdapter();
        this.device = <GPUDevice>await this.adapter.requestDevice();
        this.context = <GPUCanvasContext>this.canvas.getContext("webgpu");
        this.context.configure({
            format: this.format,
            device: this.device,
            alphaMode: "opaque",
        });
    }

    create_assets() {
        // Colour buffer
        this.colour_buffer = this.device.createTexture({
            format: "rgba8unorm",
            size: {
                width: this.canvas.width,
                height: this.canvas.height,
            },
            usage:
                GPUTextureUsage.COPY_DST | // Can be target of copy operations
                GPUTextureUsage.STORAGE_BINDING | // Mutable in compute shader
                GPUTextureUsage.TEXTURE_BINDING, // Can be used as a texture, with a sampler
        });
        this.colour_buffer_view = this.colour_buffer.createView();

        // Scene buffer
        this.scene_buffer = this.device.createBuffer({
            size: 64,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        });

        // Sphere buffer
        this.sphere_buffer = this.device.createBuffer({
            size: 32 * this.scene.spheres.length,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
        });

        // Node buffer
        this.node_buffer = this.device.createBuffer({
            size: 32 * this.scene.node_count,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
        });

        // Sphere indices buffer
        this.sphere_indices_buffer = this.device.createBuffer({
            size: 4 * this.scene.sphere_indices.length,
            usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
        });

        // Sampler
        const sampler_descriptor: GPUSamplerDescriptor = {
            addressModeU: "repeat",
            addressModeV: "repeat",
            magFilter: "linear",
            minFilter: "nearest",
            mipmapFilter: "nearest",
            maxAnisotropy: 1,
        };
        this.sampler = this.device.createSampler(sampler_descriptor);
    }

    make_pipeline() {
        const ray_tracer_bind_group_layout: GPUBindGroupLayout = this.device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.COMPUTE,
                    storageTexture: {
                        format: "rgba8unorm",
                        access: "write-only",
                        viewDimension: "2d",
                    },
                },
                {
                    binding: 1,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: {
                        type: "uniform",
                    },
                },
                {
                    binding: 2,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: {
                        type: "read-only-storage",
                        hasDynamicOffset: false,
                    },
                },
                {
                    binding: 3,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: {
                        type: "read-only-storage",
                        hasDynamicOffset: false,
                    },
                },
                {
                    binding: 4,
                    visibility: GPUShaderStage.COMPUTE,
                    buffer: {
                        type: "read-only-storage",
                        hasDynamicOffset: false,
                    },
                },
            ],
        });
        this.ray_tracer_bind_group = this.device.createBindGroup({
            layout: ray_tracer_bind_group_layout,
            entries: [
                {
                    binding: 0,
                    resource: this.colour_buffer_view,
                },
                {
                    binding: 1,
                    resource: {
                        buffer: this.scene_buffer,
                    },
                },
                {
                    binding: 2,
                    resource: {
                        buffer: this.sphere_buffer,
                    },
                },
                {
                    binding: 3,
                    resource: {
                        buffer: this.node_buffer,
                    },
                },
                {
                    binding: 4,
                    resource: {
                        buffer: this.sphere_indices_buffer,
                    },
                },
            ],
        });
        const ray_tracer_pipeline_layout: GPUPipelineLayout = this.device.createPipelineLayout({
            bindGroupLayouts: [ray_tracer_bind_group_layout],
        });
        this.ray_tracer_pipeline = this.device.createComputePipeline({
            layout: ray_tracer_pipeline_layout,
            compute: {
                module: this.device.createShaderModule({
                    code: ray_tracer_kernel,
                }),
                entryPoint: "main",
            },
        });

        const display_bind_group_layout: GPUBindGroupLayout = this.device.createBindGroupLayout({
            entries: [
                {
                    binding: 0,
                    visibility: GPUShaderStage.FRAGMENT,
                    sampler: {},
                },
                {
                    binding: 1,
                    visibility: GPUShaderStage.FRAGMENT,
                    texture: {},
                },
            ],
        });
        this.display_bind_group = this.device.createBindGroup({
            layout: display_bind_group_layout,
            entries: [
                {
                    binding: 0,
                    resource: this.sampler,
                },
                {
                    binding: 1,
                    resource: this.colour_buffer_view,
                },
            ],
        });
        const display_pipeline_layout: GPUPipelineLayout = this.device.createPipelineLayout({
            bindGroupLayouts: [display_bind_group_layout],
        });
        this.display_pipeline = this.device.createRenderPipeline({
            layout: display_pipeline_layout,
            vertex: {
                module: this.device.createShaderModule({
                    code: display_shader,
                }),
                entryPoint: "vs_main",
            },
            primitive: {
                topology: "triangle-list",
            },
            fragment: {
                module: this.device.createShaderModule({
                    code: display_shader,
                }),
                entryPoint: "fs_main",
                targets: [
                    {
                        format: this.format,
                    },
                ],
            },
        });
    }

    prepare_scene() {
        this.lambda += 0.01;
        if (this.lambda > 2.0 * Math.PI) {
            this.lambda -= 2.0 * Math.PI;
        }
        let x = 50.0 * Math.sin(this.lambda);
        let y = 50.0 * Math.cos(this.lambda);
        this.scene.camera.position[0] = x;
        this.scene.camera.position[1] = 0.0;
        this.scene.camera.position[2] = y;

        const { theta, phi } = position_and_target_to_spherical(this.scene.camera.position, [0.0, 0.0, 0.0]);
        this.scene.camera.theta = theta;
        this.scene.camera.phi = phi;
        this.scene.camera.recalculate_vectors();

        const data = new Float32Array([
            this.scene.camera.position[0],
            this.scene.camera.position[1],
            this.scene.camera.position[2],
            0.0,
            this.scene.camera.forward[0],
            this.scene.camera.forward[1],
            this.scene.camera.forward[2],
            0.0,
            this.scene.camera.right[0],
            this.scene.camera.right[1],
            this.scene.camera.right[2],
            0.0,
            this.scene.camera.up[0],
            this.scene.camera.up[1],
            this.scene.camera.up[2],
            1.0 * this.scene.spheres.length,
        ]);
        this.device.queue.writeBuffer(this.scene_buffer, 0, data, 0, data.length);

        const sphere_data = new Float32Array(this.scene.spheres.length * 8);
        for (let i = 0; i < this.scene.spheres.length; i++) {
            const node = this.scene.nodes[i];
            sphere_data[i * 8 + 0] = this.scene.spheres[i].centre[0];
            sphere_data[i * 8 + 1] = this.scene.spheres[i].centre[1];
            sphere_data[i * 8 + 2] = this.scene.spheres[i].centre[2];
            sphere_data[i * 8 + 3] = 0.0;
            sphere_data[i * 8 + 4] = this.scene.spheres[i].colour[0];
            sphere_data[i * 8 + 5] = this.scene.spheres[i].colour[1];
            sphere_data[i * 8 + 6] = this.scene.spheres[i].colour[2];
            sphere_data[i * 8 + 7] = this.scene.spheres[i].radius;
        }
        this.device.queue.writeBuffer(this.sphere_buffer, 0, sphere_data, 0, this.scene.spheres.length * 8);

        const node_data = new Float32Array(this.scene.node_count * 8);
        for (let i = 0; i < this.scene.node_count; i++) {
            const node = this.scene.nodes[i];
            node_data[i * 8 + 0] = node.min[0];
            node_data[i * 8 + 1] = node.min[1];
            node_data[i * 8 + 2] = node.min[2];
            node_data[i * 8 + 3] = node.left_child;
            node_data[i * 8 + 4] = node.max[0];
            node_data[i * 8 + 5] = node.max[1];
            node_data[i * 8 + 6] = node.max[2];
            node_data[i * 8 + 7] = node.sphere_count;
        }
        this.device.queue.writeBuffer(this.node_buffer, 0, node_data, 0, this.scene.node_count * 8);

        const sphere_index_data = new Float32Array(this.scene.spheres.length);
        for (let i = 0; i < this.scene.spheres.length; i++) {
            sphere_index_data[i] = this.scene.sphere_indices[i];
        }
        this.device.queue.writeBuffer(this.sphere_indices_buffer, 0, sphere_index_data, 0, this.scene.spheres.length);
    }

    render = (hud_callback: any) => {
        if (!this.device) {
            console.log("Loading...");
            return;
        }

        let start_time = performance.now();

        // Prepare scene
        this.prepare_scene();

        // Command encoder - must be called first
        const command_encoder: GPUCommandEncoder = this.device.createCommandEncoder();

        const ray_tracer_pass: GPUComputePassEncoder = command_encoder.beginComputePass();
        ray_tracer_pass.setPipeline(this.ray_tracer_pipeline);
        ray_tracer_pass.setBindGroup(0, this.ray_tracer_bind_group);
        ray_tracer_pass.dispatchWorkgroups(this.canvas.width, this.canvas.height, 1);
        ray_tracer_pass.end();

        const texture_view: GPUTextureView = this.context.getCurrentTexture().createView();
        const render_pass: GPURenderPassEncoder = command_encoder.beginRenderPass({
            colorAttachments: [
                {
                    view: texture_view,
                    clearValue: { r: 0.5, g: 0.0, b: 0.25, a: 1.0 },
                    loadOp: "clear",
                    storeOp: "store",
                },
            ],
        });

        render_pass.setPipeline(this.display_pipeline);
        render_pass.setBindGroup(0, this.display_bind_group);
        render_pass.draw(6, 1, 0, 0);

        render_pass.end();

        // Submit commands
        this.device.queue.submit([command_encoder.finish()]);
        this.device.queue.onSubmittedWorkDone().then(() => {
            hud_callback(performance.now() - start_time);
        });

        // Request next frame
        requestAnimationFrame(this.render.bind(this, hud_callback));
    };
}
