import { Scene } from "./scene";
import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const scene = new Scene();
const renderer = new Renderer(canvas, scene);

async function main() {
    const start = performance.now();
    await renderer.init();
    const end = performance.now();
    console.log(`Initialisation took ${end - start}ms`);
    requestAnimationFrame(renderer.render);
}

main();
