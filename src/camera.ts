import { vec3 } from "gl-matrix";

import { position_and_target_to_spherical } from "./util";

export class Camera {
    position: Float32Array;
    theta: number;
    phi: number;

    forward: Float32Array;
    right: Float32Array;
    up: Float32Array;

    constructor(position: vec3, target: vec3) {
        this.position = new Float32Array(position);

        const { theta, phi } = position_and_target_to_spherical(position, target);
        this.theta = theta;
        this.phi = phi;

        this.recalculate_vectors();
    }

    recalculate_vectors() {
        this.forward = new Float32Array([
            Math.cos((this.theta * 180.0) / Math.PI) * Math.cos((this.phi * 180.0) / Math.PI),
            Math.sin((this.theta * 180.0) / Math.PI) * Math.cos((this.phi * 180.0) / Math.PI),
            Math.sin((this.phi * 180.0) / Math.PI),
        ]);

        this.right = new Float32Array([0.0, 0.0, 0.0]);
        vec3.cross(this.right, this.forward, [0.0, 0.0, 1.0]);
        this.up = new Float32Array([0.0, 0.0, 0.0]);
        vec3.cross(this.up, this.right, this.forward);
    }
}
