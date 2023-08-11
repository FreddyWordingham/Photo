import { Scene } from "./scene";
import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const frame_time_label = <HTMLElement>document.getElementById("frame_time_label");
const object_count_label = <HTMLElement>document.getElementById("object_count_label");

const num_spheres = 1024 * 8;
const super_resolution = 0.25;

const scene = new Scene(num_spheres);
const renderer = new Renderer([canvas.width * super_resolution, canvas.height * super_resolution], canvas, scene);

async function main() {
    const start = performance.now();
    await renderer.init();
    const end = performance.now();
    const milliseconds = (end - start).toPrecision(3);
    console.log(`Initialisation took ${milliseconds}ms`);
    object_count_label.innerHTML = `${num_spheres}`;

    requestAnimationFrame(renderer.render);
}

main();
