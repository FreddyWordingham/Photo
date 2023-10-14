use crate::{simulation::NBody, Image};

pub struct Camera {
    background_colour: [f32; 4],
    position: [f32; 2],
    zoom: f32,
}

impl Camera {
    pub fn new(background_colour: [f32; 4], position: [f32; 2], zoom: f32) -> Self {
        Self {
            background_colour,
            position,
            zoom,
        }
    }

    fn position_to_pixel(&self, position: &[f32; 2], nrows: f32, ncols: f32) -> Option<[usize; 2]> {
        debug_assert!(nrows > 0.0);
        debug_assert!(ncols > 0.0);

        let row = ((position[1] - self.position[1]) * self.zoom) + (nrows * 0.5);
        let col = ((position[0] - self.position[0]) * self.zoom) + (ncols * 0.5);

        if row < 0.0 || row >= nrows || col < 0.0 || col >= ncols {
            return None;
        }

        Some([row as usize, col as usize])
    }

    pub fn render(&self, image: &mut Image, simulation: &NBody) {
        image.clear(self.background_colour);

        let nrows = image.nrows() as f32;
        let ncols = image.ncols() as f32;

        let mut max_speed = 0.4;

        for (pos, [vx, vy]) in simulation.positions().iter().zip(simulation.velocities()) {
            if let Some([row, col]) = self.position_to_pixel(pos, nrows, ncols) {
                let r = vx.hypot(*vy) / max_speed;

                image.set_pixel(row, col, [r, r, r, 1.0]);
            }
        }
    }
}
