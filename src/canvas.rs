use image::{ImageBuffer, Rgba};

/// Stores the pixels of an image.
/// Suitable for writing to a file or rendering to the screen.
#[derive(Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        for chunk in pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }

        Self {
            width,
            height: height,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn render(&self, target: &mut [u8]) {
        target.copy_from_slice(&self.pixels);
    }

    pub fn clear_background(&mut self, col: [u8; 4]) {
        for chunk in self.pixels.chunks_exact_mut(4) {
            chunk.copy_from_slice(&col);
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, col: [u8; 4]) {
        let index = 4 * (y * self.width + x) as usize;
        self.pixels[index..index + 4].copy_from_slice(&col);
    }

    pub fn draw_circle(&mut self, x: u32, y: u32, radius: u32, col: [u8; 4]) {
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

                let index = 4 * (y_pos as usize * self.width as usize + x_pos as usize);
                self.pixels[index..index + 4].copy_from_slice(&col);
            }
        }
    }

    pub fn combine(&mut self, other: &Self) {
        for (i, pixel) in self.pixels.iter_mut().enumerate() {
            *pixel = *pixel | other.pixels[i];
        }
    }

    pub fn save_to_png(&self, path: &str) {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_raw(self.width as u32, self.height as u32, self.pixels.clone())
                .expect("Failed to create image buffer");
        img.save(path).expect("Failed to save image");
    }
}
