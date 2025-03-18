use num_traits::NumCast;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn from_index<T: NumCast>(index: T) -> Self {
        let i = index.to_usize().unwrap();
        match i {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => panic!("Invalid index"),
        }
    }

    pub fn index<T: NumCast>(self) -> T {
        let i = match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        NumCast::from(i).unwrap()
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Direction::North => "N",
            Direction::East => "E",
            Direction::South => "S",
            Direction::West => "W",
        };
        write!(f, "{}", s)
    }
}

pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];
