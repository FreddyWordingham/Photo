export class Sphere {
    centre: Float32Array;
    radius: number;
    colour: Float32Array;

    constructor(centre: number[], radius: number, colour: number[]) {
        this.centre = new Float32Array(centre);
        this.radius = radius;
        this.colour = new Float32Array(colour);
    }
}
