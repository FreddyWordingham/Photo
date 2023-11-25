use palette::LinSrgba;
/// Sample the scene.
pub struct Sample {
    pub colour: LinSrgba,
}

impl Sample {
    /// Construct a new instance.
    pub fn new(colour: LinSrgba) -> Self {
        Self { colour }
    }
}

impl Default for Sample {
    fn default() -> Self {
        Self::new(LinSrgba::new(0.0, 0.0, 0.0, 1.0))
    }
}
