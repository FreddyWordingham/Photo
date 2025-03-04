use num_traits::NumCast;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Transformation {
    Identity,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    FlipDiagonal,
    FlipAntiDiagonal,
}

impl Transformation {
    pub fn index<T: NumCast>(self) -> T {
        let i = match self {
            Transformation::Identity => 0,
            Transformation::Rotate90 => 1,
            Transformation::Rotate180 => 2,
            Transformation::Rotate270 => 3,
            Transformation::FlipHorizontal => 4,
            Transformation::FlipVertical => 5,
            Transformation::FlipDiagonal => 6,
            Transformation::FlipAntiDiagonal => 7,
        };
        NumCast::from(i).unwrap()
    }
}

impl Transformation {}

pub const ALL_TRANSFORMATIONS: [Transformation; 8] = [
    Transformation::Identity,
    Transformation::Rotate90,
    Transformation::Rotate180,
    Transformation::Rotate270,
    Transformation::FlipHorizontal,
    Transformation::FlipVertical,
    Transformation::FlipDiagonal,
    Transformation::FlipAntiDiagonal,
];
