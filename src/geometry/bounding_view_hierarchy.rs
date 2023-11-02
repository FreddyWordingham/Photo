use crate::geometry::{Scene, Triangle};

const TARGET_TRIANGLE_COUNT: usize = 2;

#[derive(Debug, Clone, Copy)]
struct BVHNode {
    mins: [f32; 3],
    left_child: usize,
    maxs: [f32; 3],
    count: usize,
}

pub struct BVHBuilder {
    triangles: Vec<Triangle>,
    triangle_count: usize,
    triangle_indices: Vec<usize>,
    nodes: Vec<BVHNode>,
    nodes_used: usize,
}

impl BVHBuilder {
    pub fn bvh_data(&self) -> Vec<f32> {
        let mut data = Vec::with_capacity(8 * self.nodes.len());

        for node in &self.nodes {
            data.extend_from_slice(&node.mins);
            data.push(node.left_child as f32);
            data.extend_from_slice(&node.maxs);
            data.push(node.count as f32);
        }

        data
    }

    pub fn bvh_indices(&self) -> Vec<u32> {
        self.triangle_indices.iter().map(|&i| i as u32).collect()
    }

    pub fn new(scene: &Scene) -> Self {
        let triangles = scene.create_triangles();
        let triangle_count: usize = triangles.len();

        Self {
            triangles,
            triangle_count,
            triangle_indices: Vec::with_capacity(triangle_count),
            nodes: Vec::with_capacity((triangle_count * 2) - 1),
            nodes_used: 0,
        }
    }

    pub fn build(&mut self) {
        println!("building...");

        self.triangle_indices = (0..self.triangle_count).collect();
        self.nodes = vec![
            BVHNode {
                mins: [0.0, 0.0, 0.0],
                left_child: 0,
                maxs: [0.0, 0.0, 0.0],
                count: 0,
            };
            (self.triangle_count * 2) - 1
        ];

        self.nodes[0].left_child = 0;
        self.nodes[0].count = self.triangle_count;
        self.nodes_used = 1;

        self.update_bounds(0);
        self.subdivide(0);

        self.nodes.truncate(self.nodes_used);
    }

    fn update_bounds(&mut self, index: usize) {
        self.nodes[index].mins = [999999.0, 999999.0, 999999.0];
        self.nodes[index].maxs = [-999999.0, -999999.0, -999999.0];

        for i in 0..self.nodes[index].count {
            let triangle = self.triangles[self.triangle_indices[i]];

            for point in triangle.positions() {
                for (n, &value) in point.iter().enumerate() {
                    if value < self.nodes[index].mins[n] {
                        self.nodes[index].mins[n] = value;
                    }

                    if value > self.nodes[index].maxs[n] {
                        self.nodes[index].maxs[n] = value;
                    }
                }
            }
        }
    }

    fn subdivide(&mut self, index: usize) {
        if self.nodes[index].count <= TARGET_TRIANGLE_COUNT {
            return;
        }

        let extent = [
            self.nodes[index].maxs[0] - self.nodes[index].mins[0],
            self.nodes[index].maxs[1] - self.nodes[index].mins[1],
            self.nodes[index].maxs[2] - self.nodes[index].mins[2],
        ];
        let axis = if extent[0] > extent[1] && extent[0] > extent[2] {
            0
        } else if extent[1] > extent[2] {
            1
        } else {
            2
        };

        let split_position = self.nodes[index].mins[axis] + (extent[axis] * 0.5);

        let mut i = self.nodes[index].left_child;
        let mut j = i + self.nodes[index].count - 1;

        while i <= j {
            if self.triangles[self.triangle_indices[i]].centre()[axis] < split_position {
                i += 1;
            } else {
                let temp = self.triangle_indices[i];
                self.triangle_indices[i] = self.triangle_indices[j];
                self.triangle_indices[j] = temp;
                j -= 1;
            }
        }

        let left_count = i - self.nodes[index].left_child;
        if (left_count == 0) || (left_count == self.nodes[index].count) {
            return;
        }

        let left_child_index = self.nodes_used;
        self.nodes_used += 1;
        let right_child_index = self.nodes_used;
        self.nodes_used += 1;

        self.nodes[left_child_index].left_child = self.nodes[index].left_child;
        self.nodes[left_child_index].count = left_count;

        self.nodes[right_child_index].left_child = i;
        self.nodes[right_child_index].count = self.nodes[index].count - left_count;

        self.nodes[index].left_child = left_child_index;
        self.nodes[index].count = 0;

        self.update_bounds(left_child_index);
        self.update_bounds(right_child_index);
        self.subdivide(left_child_index);
        self.subdivide(right_child_index);
    }
}
