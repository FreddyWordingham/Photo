use rand::Rng;

#[derive(Clone, Debug)]
pub struct NBodyInit {
    pub massive_positions: Vec<[f32; 3]>,
    pub massive_velocities: Vec<[f32; 3]>,
    pub massive_masses: Vec<f32>,
    pub ghost_positions: Vec<[f32; 3]>,
    pub ghost_velocities: Vec<[f32; 3]>,
}

impl Default for NBodyInit {
    fn default() -> Self {
        Self {
            massive_positions: Vec::new(),
            massive_velocities: Vec::new(),
            massive_masses: Vec::new(),
            ghost_positions: Vec::new(),
            ghost_velocities: Vec::new(),
        }
    }
}

impl NBodyInit {
    pub fn num_massive_bodies(&self) -> usize {
        debug_assert!(self.is_valid());
        self.massive_positions.len()
    }

    pub fn num_ghost_bodies(&self) -> usize {
        debug_assert!(self.is_valid());
        self.ghost_positions.len()
    }

    pub fn is_valid(&self) -> bool {
        !self.massive_positions.is_empty()
            && self.massive_positions.len() == self.massive_velocities.len()
            && self.massive_positions.len() == self.massive_masses.len()
            && self.ghost_positions.len() == self.ghost_velocities.len()
    }

    pub fn add_massive_particle(&mut self, position: [f32; 3], velocity: [f32; 3], mass: f32) {
        self.massive_positions.push(position);
        self.massive_velocities.push(velocity);
        self.massive_masses.push(mass);
    }

    pub fn add_massive_disk<R: Rng>(
        &mut self,
        rng: &mut R,
        centre: [f32; 3],
        radius: f32,
        num: usize,
        total_mass: f32,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num > 0);
        debug_assert!(total_mass > 0.0);

        self.massive_positions.reserve_exact(num);
        self.massive_velocities.reserve_exact(num);
        self.massive_masses.reserve_exact(num);

        for _ in 0..num {
            let mut dx = rng.gen_range(-radius..radius);
            let mut dy = rng.gen_range(-radius..radius);

            while dx * dx + dy * dy > radius * radius {
                dx = rng.gen_range(-radius..radius);
                dy = rng.gen_range(-radius..radius);
            }

            let position = [centre[0] + dx, centre[1] + dy, centre[2]];
            let velocity = [0.0, 0.0, 0.0];

            self.massive_positions.push(position);
            self.massive_velocities.push(velocity);
            self.massive_masses.push(total_mass / num as f32);
        }
    }

    pub fn add_massive_field<R: Rng>(
        &mut self,
        rng: &mut R,
        centre: [f32; 3],
        radius: f32,
        num: usize,
        total_mass: f32,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num > 0);

        self.massive_positions.reserve_exact(num);
        self.massive_velocities.reserve_exact(num);
        self.massive_masses.reserve_exact(num);

        for _ in 0..num {
            let mut dx = rng.gen_range(-radius..radius);
            let mut dy = rng.gen_range(-radius..radius);
            let mut dz = rng.gen_range(-radius..radius);

            while dx * dx + dy * dy + dz * dz > radius * radius {
                dx = rng.gen_range(-radius..radius);
                dy = rng.gen_range(-radius..radius);
                dz = rng.gen_range(-radius..radius);
            }

            let position = [centre[0] + dx, centre[1] + dy, centre[2] + dz];
            let velocity = [0.0, 0.0, 0.0];

            self.massive_positions.push(position);
            self.massive_velocities.push(velocity);
            self.massive_masses.push(total_mass / num as f32);
        }
    }

    pub fn add_ghost_field<R: Rng>(
        &mut self,
        rng: &mut R,
        centre: [f32; 3],
        radius: f32,
        num: usize,
    ) {
        debug_assert!(radius > 0.0);
        debug_assert!(num > 0);

        self.ghost_positions.reserve_exact(num);
        self.ghost_velocities.reserve_exact(num);

        for _ in 0..num {
            let mut dx = rng.gen_range(-radius..radius);
            let mut dy = rng.gen_range(-radius..radius);
            let mut dz = rng.gen_range(-radius..radius);

            while dx * dx + dy * dy + dz * dz > radius * radius {
                dx = rng.gen_range(-radius..radius);
                dy = rng.gen_range(-radius..radius);
                dz = rng.gen_range(-radius..radius);
            }

            let position = [centre[0] + dx, centre[1] + dy, centre[2] + dz];
            let velocity = [0.0, 0.0, 0.0];

            self.ghost_positions.push(position);
            self.ghost_velocities.push(velocity);
        }
    }
}
