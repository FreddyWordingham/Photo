use enterpolation::{linear::Linear, Generator, Identity, Sorted};
use palette::LinSrgba;

#[derive(Debug, Clone)]
pub struct Gradient {
    pub colours: Linear<Sorted<Vec<f32>>, Vec<LinSrgba>, Identity>,
}

impl Gradient {
    /// Create a new gradient from a list of RGBA colours.
    pub fn new(colours: Vec<u32>) -> Self {
        let new = Self {
            colours: Linear::builder()
                .elements(
                    colours
                        .iter()
                        .map(|colour| {
                            let red = ((colour >> 24) & 0xFF) as f32 / 255.0;
                            let green = ((colour >> 16) & 0xFF) as f32 / 255.0;
                            let blue = ((colour >> 8) & 0xFF) as f32 / 255.0;
                            let alpha = (colour & 0xFF) as f32 / 255.0;

                            LinSrgba::new(red, green, blue, alpha)
                        })
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
        };

        new
    }

    /// Sample the gradient for a colour at a given point in the range [0, 1].
    pub fn sample(&self, t: f32) -> LinSrgba {
        debug_assert!(t >= 0.0 && t <= 1.0);
        self.colours.sample([t]).collect::<Vec<_>>()[0]
    }
}
