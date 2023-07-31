import { Renderer } from "./renderer";

const canvas: HTMLCanvasElement = <HTMLCanvasElement>document.getElementById("gfx_main");

const renderer = new Renderer(canvas);
renderer.init();

for (let i = 0; i < 100; i++) {
    requestAnimationFrame(renderer.render);
}
