import { Scene } from "./scene";
import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const fps_label = <HTMLElement>document.getElementById("fps_label");
const object_count_label = <HTMLElement>document.getElementById("object_count_label");

const num_spheres = 64;

const scene = new Scene(num_spheres);
const renderer = new Renderer(canvas, scene);

let hundred_frame_average = 0.0;
function hud_callback(render_time: number) {
    hundred_frame_average = (hundred_frame_average * 99.0 + render_time) / 100.0;
    const fps = (1000.0 / hundred_frame_average).toFixed(0);
    fps_label.innerHTML = `${fps}`;
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
