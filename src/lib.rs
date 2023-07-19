mod run;
mod state;
mod util;
mod vertex;

pub use {run::start, vertex::Vertex};

const VERTICES_A: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.4131759, 1.0 - 0.99240386],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.0048659444, 1.0 - 0.56958647],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.28081453, 1.0 - 0.05060294],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.85967, 1.0 - 0.1526709],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.9414737, 1.0 - 0.7347359],
    },
];
const INDICES_A: &[u16] = &[
    0, 1, 4, //
    1, 2, 4, //
    2, 3, 4, //
];

const VERTICES_B: &[Vertex] = &[
    Vertex {
        position: [-0.0, 0.5, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.4131759, 1.0 - 0.99240386],
    },
    Vertex {
        position: [-0.5, 0.1, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.0048659444, 1.0 - 0.56958647],
    },
    Vertex {
        position: [-0.2, -0.4, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.28081453, 1.0 - 0.05060294],
    },
    Vertex {
        position: [0.4, -0.3, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.85967, 1.0 - 0.1526709],
    },
    Vertex {
        position: [0.5, 0.2, 0.0],
        colour: [0.5, 0.0, 0.5],
        texture_coords: [0.9414737, 1.0 - 0.7347359],
    },
];
const INDICES_B: &[u16] = &[
    0, 1, 4, //
    1, 2, 4, //
    2, 3, 4, //
];
