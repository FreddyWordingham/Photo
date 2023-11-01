#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AABB {
    mins: [f32; 3],
    maxs: [f32; 3],
}

impl AABB {
    pub fn new(mins: [f32; 3], maxs: [f32; 3]) -> Self {
        debug_assert!(mins.iter().zip(maxs.iter()).all(|(min, max)| min <= max));

        Self { mins, maxs }
    }

    pub fn is_valid(&self) -> bool {
        self.mins
            .iter()
            .zip(self.maxs.iter())
            .all(|(min, max)| min <= max)
    }

    pub fn mins(&self) -> [f32; 3] {
        self.mins
    }

    pub fn maxs(&self) -> [f32; 3] {
        self.maxs
    }

    pub fn expand_to_accommodate(&mut self, other: &Self) {
        for (n, min) in self.mins.iter_mut().enumerate() {
            if other.mins[n] < *min {
                *min = other.mins[n];
            }
        }

        for (n, max) in self.maxs.iter_mut().enumerate() {
            if other.maxs[n] > *max {
                *max = other.maxs[n];
            }
        }
    }
}
