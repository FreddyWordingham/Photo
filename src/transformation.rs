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
