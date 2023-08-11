import { Scene } from "./scene";
import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const frame_time_label = <HTMLElement>document.getElementById("frame_time_label");
const object_count_label = <HTMLElement>document.getElementById("object_count_label");

const num_spheres = 1024 * 8;
const super_resolution = 0.5;

const scene = new Scene(num_spheres);
const renderer = new Renderer([canvas.width * super_resolution, canvas.height * super_resolution], canvas, scene);

let prev_frame_times: number[] = [];
function hud_callback(render_time: number) {
    prev_frame_times.push(render_time);
    const ave_frame_time = prev_frame_times.reduce((a, b) => a + b, 0) / prev_frame_times.length;
    if (prev_frame_times.length > 100) {
        prev_frame_times.shift();
    }
    frame_time_label.innerHTML = `${ave_frame_time.toFixed(0)}ms`;
}

async function main() {
    const start = performance.now();
    await renderer.init();
    const end = performance.now();
    const milliseconds = (end - start).toPrecision(3);
    console.log(`Initialisation took ${milliseconds}ms`);
    object_count_label.innerHTML = `${num_spheres}`;
    requestAnimationFrame(renderer.render.bind(renderer, hud_callback));
}

main();
