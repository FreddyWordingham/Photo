import { vec3 } from "gl-matrix";

import { Camera } from "./camera";
import { Sphere } from "./sphere";

const NUM_SPHERES = 32;

export class Scene {
    camera: Camera;
    spheres: Sphere[];

    constructor() {
        this.spheres = Scene.create_spheres();
        this.camera = Scene.create_camera();
    }

    private static create_spheres(): Sphere[] {
        return new Array(NUM_SPHERES).map((s) => {
            const centre = [3.0 + 7.0 * Math.random(), -5.0 + 10.0 * Math.random(), -5.0 + 10.0 * Math.random()];
            const radius = 0.1 + 1.9 * Math.random();
            const colour = [0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random()];
            return new Sphere(centre, radius, colour);
        });
    }

    private static create_camera(): Camera {
        const position: vec3 = [0.0, 0.0, 0.0];
        const target: vec3 = [1.0, 0.0, 0.0];
        return new Camera(position, target);
    }
}
