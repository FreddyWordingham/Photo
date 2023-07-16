mod run;
mod state;
mod util;
mod vertex;

pub use {run::start, vertex::Vertex};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        colour: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        colour: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        colour: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        colour: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        colour: [0.5, 0.0, 0.5],
    },
];

const INDICES: &[u16] = &[
    0, 1, 4, //
    1, 2, 4, //
    2, 3, 4, //
];
