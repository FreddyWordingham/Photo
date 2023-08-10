import { Scene } from "./scene";
import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const frame_time_label = <HTMLElement>document.getElementById("frame_time_label");
const object_count_label = <HTMLElement>document.getElementById("object_count_label");

const num_spheres = 1024 / 8;

const scene = new Scene(num_spheres);
const renderer = new Renderer(canvas, scene);

let prev_frame_times: number[] = [];
function hud_callback(render_time: number) {
    prev_frame_times.push(render_time);
    if (prev_frame_times.length > 10) {
        prev_frame_times.shift();
    }
    const fps = Math.round(1000 / (prev_frame_times.reduce((a, b) => a + b, 0) / prev_frame_times.length));
    frame_time_label.innerHTML = `${fps / 1000}ms`;
}

async function main() {
    const start = performance.now();
    await renderer.init();
    const end = performance.now();
    console.log(`Initialisation took ${end - start}ms`);
    object_count_label.innerHTML = `${num_spheres}`;
    requestAnimationFrame(renderer.render.bind(renderer, hud_callback));
}

main();
