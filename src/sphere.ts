import { vec3 } from "gl-matrix";

export class Sphere {
    centre: Float32Array;
    radius: number;
    colour: Float32Array;

    constructor(centre: vec3, radius: number, colour: vec3) {
        this.centre = new Float32Array(centre);
        this.radius = radius;
        this.colour = new Float32Array(colour);
    }
}
