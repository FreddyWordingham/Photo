import { vec3 } from "gl-matrix";

export class Node {
    min: vec3;
    left_child: number;
    max: vec3;
    sphere_count: number;
}
