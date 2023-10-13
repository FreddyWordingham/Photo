use rand::Rng;

pub struct NBody {
    positions: Vec<[f32; 2]>,
    velocities: Vec<[f32; 2]>,
    masses: Vec<f32>,
}

impl NBody {
    pub fn new<R: Rng>(rng: &mut R, num_bodies: usize) -> Self {
        debug_assert!(num_bodies > 0);

        let mut positions = Vec::with_capacity(num_bodies);
        let mut velocities = Vec::with_capacity(num_bodies);
        let mut masses = Vec::with_capacity(num_bodies);
        for _ in 0..num_bodies {
            let mut px = rng.gen_range(-1.0..1.0);
            let mut py = rng.gen_range(-1.0..1.0);
            let mut r = px * px + py * py;

            while r >= 1.0 || r < 0.9 {
                px = rng.gen_range(-1.0..1.0);
                py = rng.gen_range(-1.0..1.0);
                r = px * px + py * py;
            }

            let theta = rng.gen_range(0.0..(2.0 * std::f32::consts::PI));
            let speed = 1.0e1;
            let vx = speed * theta.cos();
            let vy = speed * theta.sin();

            positions.push([px, py]);
            velocities.push([vx, vy]);
            masses.push(1.0e-3);
        }

        Self {
            positions,
            velocities,
            masses,
        }
    }

    pub fn positions(&self) -> &[[f32; 2]] {
        &self.positions
    }

    pub fn velocities(&self) -> &[[f32; 2]] {
        &self.velocities
    }

    pub fn masses(&self) -> &[f32] {
        &self.masses
    }

    fn calculate_forces(&self) -> Vec<[f32; 2]> {
        let mut forces = vec![[0.0, 0.0]; self.positions.len()];

        for i in 0..self.positions.len() {
            let (head, tail) = self.positions.split_at(i + 1);
            let pi = &head[i];
            let mi = self.masses[i];

            for (j, pj) in tail.iter().enumerate() {
                let j = i + 1 + j;
                let mj = self.masses[j];

                let dx = pj[0] - pi[0];
                let dy = pj[1] - pi[1];
                let r2 = dx * dx + dy * dy;
                let r = r2.sqrt();
                let f_over_r = mi * mj / (r * r2);
                let fx = f_over_r * dx;
                let fy = f_over_r * dy;

                forces[i][0] += fx;
                forces[i][1] += fy;
                forces[j][0] -= fx;
                forces[j][1] -= fy;
            }
        }

        forces
    }

    fn update_velocities(&mut self, forces: &[[f32; 2]], dt: f32) {
        for ([vx, vy], ([fx, fy], m)) in self
            .velocities
            .iter_mut()
            .zip(forces.iter().zip(&self.masses))
        {
            *vx += fx * dt / m;
            *vy += fy * dt / m;
        }
    }

    fn update_positions(&mut self, dt: f32) {
        for ([px, py], [vx, vy]) in self.positions.iter_mut().zip(self.velocities.iter()) {
            *px += vx * dt;
            *py += vy * dt;
        }
    }

    pub fn step(&mut self, dt: f32) {
        let forces = self.calculate_forces();
        self.update_velocities(&forces, dt);
        self.update_positions(dt);
    }
}
