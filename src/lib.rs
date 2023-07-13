mod run;
mod state;
mod util;
mod vertex;

pub use {run::start, vertex::Vertex};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.3, 0.5, 0.0],
        colour: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        colour: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        colour: [0.0, 0.0, 1.0],
    },
];
