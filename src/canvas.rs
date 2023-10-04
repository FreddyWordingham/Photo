use image::{ImageBuffer, Rgba};

/// Stores the pixels of an image.
/// Suitable for writing to a file or rendering to the screen.
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = vec![0u8; width * height * 4];
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
        }

        Self {
            width,
            height: height,
            pixels,
        }
    }

    pub fn render(&self, target: &mut [u8]) {
        target.copy_from_slice(&self.pixels);
    }

    pub fn clear_background(&mut self, col: [u8; 4]) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&col);
        }
    }

    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, col: [u8; 4]) {
        let radius_squared = (radius as isize).pow(2);
        for dy in -(radius as isize) as isize..=radius as isize {
            let y_pos = y as isize + dy;
            if y_pos < 0 || y_pos >= self.width as isize {
                continue;
            }

            let dx_max = ((radius_squared - dy.pow(2)) as f32).sqrt() as isize;
            for dx in -dx_max..=dx_max {
                let x_pos = x as isize + dx;
                if x_pos < 0 || x_pos >= self.width as isize {
                    continue;
                }

                let index = 4 * (y_pos as usize * self.width + x_pos as usize);
                self.pixels[index..index + 4].copy_from_slice(&col);
            }
        }
    }

    pub fn save_to_png(&self, path: &str) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width as u32, self.height as u32, self.pixels.clone())
                .expect("Failed to create image buffer");
        img.save(path).expect("Failed to save image");
    }
}
