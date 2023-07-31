export class Renderer {
    // Device/Context objects
    canvas: HTMLCanvasElement;
    format: GPUTextureFormat;
    adapter!: GPUAdapter;
    device!: GPUDevice;
    context: GPUCanvasContext;

    // // Assets
    // colour_buffer: GPUTexture;
    // colour_buffer_view: GPUTextureView;
    // sampler: GPUSampler;

    // // Pipeline
    // ray_tracing_pipeline: GPUComputePipeline;
    // ray_tracing_bind_group: GPUBindGroup;
    // screen_pipeline: GPURenderPipeline;
    // screen_bind_group: GPUBindGroup;

    constructor(canvas: HTMLCanvasElement) {
        this.canvas = canvas;
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
            device: this.device,
            format: this.format,
            alphaMode: "opaque",
        });
    }

    async create_assets() {}

    async make_pipeline() {}
}
