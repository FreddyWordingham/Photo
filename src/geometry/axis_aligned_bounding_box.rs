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

    pub fn mins(&self) -> [f32; 3] {
        self.mins
    }

    pub fn maxs(&self) -> [f32; 3] {
        self.maxs
    }

    pub fn all_vertices(&self) -> [[f32; 3]; 8] {
        [
            [self.mins[0], self.mins[1], self.mins[2]],
            [self.mins[0], self.mins[1], self.maxs[2]],
            [self.mins[0], self.maxs[1], self.mins[2]],
            [self.mins[0], self.maxs[1], self.maxs[2]],
            [self.maxs[0], self.mins[1], self.mins[2]],
            [self.maxs[0], self.mins[1], self.maxs[2]],
            [self.maxs[0], self.maxs[1], self.mins[2]],
            [self.maxs[0], self.maxs[1], self.maxs[2]],
        ]
    }

    pub fn is_valid(&self) -> bool {
        self.mins
            .iter()
            .zip(self.maxs.iter())
            .all(|(min, max)| min <= max)
    }

    pub fn smallest_axis(&self) -> usize {
        let widths = self.widths();

        if widths[0] <= widths[1] && widths[0] <= widths[2] {
            0
        } else if widths[1] <= widths[0] && widths[1] <= widths[2] {
            1
        } else {
            2
        }
    }

    pub fn longest_axis(&self) -> usize {
        let widths = self.widths();

        if widths[0] >= widths[1] && widths[0] >= widths[2] {
            0
        } else if widths[1] >= widths[0] && widths[1] >= widths[2] {
            1
        } else {
            2
        }
    }

    pub fn mids(&self) -> [f32; 3] {
        [
            (self.mins[0] + self.maxs[0]) * 0.5,
            (self.mins[1] + self.maxs[1]) * 0.5,
            (self.mins[2] + self.maxs[2]) * 0.5,
        ]
    }

    pub fn widths(&self) -> [f32; 3] {
        [
            self.maxs[0] - self.mins[0],
            self.maxs[1] - self.mins[1],
            self.maxs[2] - self.mins[2],
        ]
    }

    pub fn half_widths(&self) -> [f32; 3] {
        [
            (self.maxs[0] - self.mins[0]) * 0.5,
            (self.maxs[1] - self.mins[1]) * 0.5,
            (self.maxs[2] - self.mins[2]) * 0.5,
        ]
    }
}
