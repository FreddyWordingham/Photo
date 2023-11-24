use enterpolation::{linear::Linear, Generator, Identity, Sorted};
use palette::LinSrgba;

pub struct Gradient {
    pub colours: Linear<Sorted<Vec<f32>>, Vec<LinSrgba>, Identity>,
}

impl Gradient {
    /// Create a new gradient from a list of RGBA colours.
    pub fn new(colours: Vec<[f32; 4]>) -> Self {
        Self {
            colours: Linear::builder()
                .elements(
                    colours
                        .iter()
                        .map(|colour| LinSrgba::new(colour[0], colour[1], colour[2], colour[3]))
                        .collect::<Vec<_>>(),
                )
                .knots(
                    colours
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i as f32 / (colours.len() - 1) as f32)
                        .collect::<Vec<_>>(),
                )
                .build()
                .expect("Failed to build colour gradient"),
        }
    }

    /// Sample the gradient for a colour at a given point in the range [0, 1].
    pub fn sample(&self, t: f32) -> LinSrgba {
        debug_assert!(t >= 0.0 && t <= 1.0);
        self.colours.sample([t]).collect::<Vec<_>>()[0]
    }
}
