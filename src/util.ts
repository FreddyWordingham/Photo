import { vec3 } from "gl-matrix";

// Given a position and a target, compute the direction vector
export function position_and_target_to_direction(position: vec3, target: vec3): vec3 {
    const dir = vec3.create();
    vec3.subtract(dir, target, position);
    return dir;
}

// Given a direction vector, compute the spherical coordinates
export function direction_to_spherical(dir: vec3): { theta: number; phi: number } {
    const length = vec3.length(dir);
    const theta = Math.atan2(dir[1], dir[0]);
    const phi = Math.acos(dir[2] / length);
    return { theta, phi };
}

// Given a position and a target, compute the spherical coordinates
export function position_and_target_to_spherical(position: vec3, target: vec3): { theta: number; phi: number } {
    const dir = position_and_target_to_direction(position, target);
    return direction_to_spherical(dir);
}
