use num_traits::{Float, NumCast};

/// Helper trait to convert normalized float values ([0,1]) to u8.
pub trait NormFloat: Float + NumCast {
    fn to_u8(self) -> u8 {
        let clamped = self.max(Self::zero()).min(Self::one());
        NumCast::from(clamped * NumCast::from(255).unwrap()).unwrap()
    }
}

impl NormFloat for f32 {}
impl NormFloat for f64 {}
