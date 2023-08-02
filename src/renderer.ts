import ray_tracer_kernel from "./shaders/ray_tracer.wgsl";
import display_shader from "./shaders/display.wgsl";

import { Scene } from "./scene";

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

    // Pipeline
    ray_tracer_pipeline: GPUComputePipeline;
    ray_tracer_bind_group: GPUBindGroup;
    display_pipeline: GPURenderPipeline;
    display_bind_group: GPUBindGroup;

    // Scene to render
    scene: Scene;

    constructor(canvas: HTMLCanvasElement, scene: Scene) {
        this.canvas = canvas;
        this.scene = scene;
    }

    async init() {
        await this.setup_device();
        await this.create_assets();
        await this.make_pipeline();
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

    async create_assets() {
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

        const parameter_buffer_descriptor: GPUBufferDescriptor = {
            size: 64,
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
        };
        this.scene_buffer = this.device.createBuffer(parameter_buffer_descriptor);
    }

    async make_pipeline() {
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
        this.device.queue.writeBuffer(
            this.scene_buffer,
            0,
            new Float32Array([this.scene.camera.position[0], this.scene.camera.position[1], this.scene.camera.position[2]]),
            0,
            3
        );
    }

    render = () => {
        if (!this.device) {
            console.log("Loading...");
            return;
        }

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

        // Request next frame
        requestAnimationFrame(this.render);
    };
}
