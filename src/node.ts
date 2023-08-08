import { vec3 } from "gl-matrix";

export class Node {
    min: vec3;
    left_child: number;
    max: vec3;
    sphere_count: number;

    constructor() {
        this.min = vec3.create();
        this.left_child = 0;
        this.max = vec3.create();
        this.sphere_count = 0;
    }
}
