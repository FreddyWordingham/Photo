import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const renderer = new Renderer(canvas);

async function main() {
    await renderer.init();
    requestAnimationFrame(renderer.render);
}

main();
