import { vec3 } from "gl-matrix";

import { Camera } from "./camera";
import { Sphere } from "./sphere";

export class Scene {
    camera: Camera;
    spheres: Sphere[];

    constructor(num_spheres: number) {
        this.spheres = Scene.create_spheres(num_spheres);
        this.camera = Scene.create_camera();
    }

    private static create_spheres(num_spheres: number): Sphere[] {
        let spheres: Sphere[] = [];
        for (let i = 0; i < num_spheres; i++) {
            const centre: vec3 = [-50.0 + 100.0 * Math.random(), -50.0 + 100.0 * Math.random(), -50.0 + 100.0 * Math.random()];
            const radius = 0.1 + 1.9 * Math.random();
            const colour: vec3 = [0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random()];
            const sp = new Sphere(centre, radius, colour);
            spheres.push(sp);
        }
        return spheres;
    }

    private static create_camera(): Camera {
        const position: vec3 = [-15.0, 0.0, 0.0];
        const target: vec3 = [0.0, 0.0, 0.0];
        return new Camera(position, target);
    }
}
