import { vec3 } from "gl-matrix";

import { Camera } from "./camera";
import { Sphere } from "./sphere";
import { Node } from "./node";

export class Scene {
    // View
    camera: Camera;
    spheres: Sphere[];

    // Bounding Volume Hierarchy
    nodes: Node[];
    sphere_indices: number[];
    node_count: number = 0;

    constructor(num_spheres: number) {
        this.spheres = Scene.create_spheres(num_spheres);
        this.camera = Scene.create_camera();
        this.build_bvh();
    }

    private static create_spheres(num_spheres: number): Sphere[] {
        let spheres: Sphere[] = [];
        let max_dist = 30.0;
        for (let i = 0; i < num_spheres; i++) {
            const centre: vec3 = [2.0 * (Math.random() - 0.5) * max_dist, 2.0 * (Math.random() - 0.5) * max_dist, 2.0 * (Math.random() - 0.5) * max_dist];
            let radius = Math.random();
            radius *= radius;
            radius *= radius;
            radius *= 3.0;
            const colour: vec3 = [0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random(), 0.3 + 0.7 * Math.random()];
            const sp = new Sphere(centre, radius, colour);
            spheres.push(sp);
        }
        return spheres;
    }

    private static create_camera(): Camera {
        const position: vec3 = [-75.0, 0.0, 0.0];
        const target: vec3 = [0.0, 0.0, 0.0];
        return new Camera(position, target);
    }

    build_bvh() {
        const start = performance.now();

        // Initialise the look-up table for the sphere indices
        this.sphere_indices = new Array<number>(this.spheres.length);
        for (let i = 0; i < this.spheres.length; i++) {
            this.sphere_indices[i] = i;
        }

        // Initialise the nodes array
        this.nodes = new Array(this.spheres.length * 2 - 1); // This can hold the maximum number of nodes
        for (let i = 0; i < 2 * this.spheres.length - 1; i++) {
            this.nodes[i] = new Node();
        }

        // Initialise the root node
        let root = this.nodes[0];
        root.left_child = 0;
        root.sphere_count = this.spheres.length;
        this.node_count = 1;

        this.update_bounds(0);
        this.subdivide(0);

        const end = performance.now();
        console.log(`BVH build took ${end - start}ms`);
    }

    update_bounds(node_index: number) {
        const node = this.nodes[node_index];
        node.min = [999999, 999999, 999999];
        node.max = [-999999, -999999, -999999];

        // TODO: Optimise to look at min and max of children instead of all spheres
        // const left_child = this.nodes[node.left_child];
        // const right_child = this.nodes[node.left_child + 1];

        for (let i = 0; i < node.sphere_count; i++) {
            const sphere = this.spheres[this.sphere_indices[node.left_child + i]];
            const axis: vec3 = [sphere.radius, sphere.radius, sphere.radius];

            var temp: vec3 = [0.0, 0.0, 0.0];
            vec3.subtract(temp, sphere.centre, axis);
            vec3.min(node.min, node.min, temp);
            vec3.add(temp, sphere.centre, axis);
            vec3.max(node.max, node.max, temp);
        }
    }

    subdivide(node_index: number) {
        var node = this.nodes[node_index];

        if (node.sphere_count <= 2) {
            return;
        }

        // Find the longest axis
        const extent: vec3 = [0.0, 0.0, 0.0];
        vec3.subtract(extent, node.max, node.min);
        let longest_axis = 0;
        if (extent[1] > extent[longest_axis]) {
            longest_axis = 1;
        }
        if (extent[2] > extent[longest_axis]) {
            longest_axis = 2;
        }

        const split_point = (node.min[longest_axis] + node.max[longest_axis]) * 0.5;

        let n = node.left_child;
        let m = n + node.sphere_count - 1;

        while (n <= m) {
            if (this.spheres[this.sphere_indices[n]].centre[longest_axis] < split_point) {
                n += 1;
            } else {
                const temp = this.sphere_indices[n];
                this.sphere_indices[n] = this.sphere_indices[m];
                this.sphere_indices[m] = temp;
                m -= 1;
            }
        }

        let left_child = n - node.left_child;
        if (left_child == 0 || left_child == node.sphere_count) {
            return;
        }

        const left_child_index = this.node_count;
        this.node_count += 1;
        const right_child_index = this.node_count;
        this.node_count += 1;

        this.nodes[left_child_index].left_child = node.left_child;
        this.nodes[left_child_index].sphere_count = left_child;

        this.nodes[right_child_index].left_child = n;
        this.nodes[right_child_index].sphere_count = node.sphere_count - left_child;

        node.left_child = left_child_index;
        node.sphere_count = 0;

        this.update_bounds(left_child_index);
        this.update_bounds(right_child_index);
        this.subdivide(left_child_index);
        this.subdivide(right_child_index);
    }
}
