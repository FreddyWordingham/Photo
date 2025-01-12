use std::{fs::File, io::BufWriter, path::Path};

use ndarray::{Array3, Axis};
use num_traits::{Float, FromPrimitive};
use png::{ColorType, Decoder, Encoder};

use crate::image::Image;

impl<T> Image for Array3<T>
where
    T: Float + FromPrimitive,
{
    fn save<P: AsRef<Path>>(&self, path: P) {
        assert!(
            self.shape()[2] == 3 || self.shape()[2] == 4,
            "Color image must have 3 or 4 channels (RGB or RGBA)."
        );
        assert!(self.iter().all(|&x| x >= T::zero() && x <= T::one()));

        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);

        let (height, width, channels) = (
            self.shape()[0] as u32,
            self.shape()[1] as u32,
            self.shape()[2],
        );
        let color_type = match channels {
            3 => ColorType::Rgb,
            4 => ColorType::Rgba,
            _ => unreachable!(),
        };

        let mut encoder = Encoder::new(&mut writer, width, height);
        encoder.set_color(color_type);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        // Collect each row then flatten
        let data = self
            .axis_iter(Axis(0))
            .rev()
            .flat_map(|row| {
                row.iter()
                    .map(|&x| (x * T::from(255.0).unwrap()).to_u8().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<u8>>();

        writer.write_image_data(&data).unwrap();
    }

    fn load<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path).unwrap();
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();

        // Accept only RGB or RGBA
        let channels = match info.color_type {
            ColorType::Rgb => 3,
            ColorType::Rgba => 4,
            _ => panic!("Only RGB or RGBA images are supported."),
        };

        let width = info.width as usize;
        let height = info.height as usize;
        let data = buf[..info.buffer_size()]
            .iter()
            .map(|&x| T::from_u8(x).unwrap() / T::from_u8(255).unwrap())
            .collect::<Vec<T>>();

        let mut array = Array3::<T>::zeros((height, width, channels));
        for (idx, val) in data.into_iter().enumerate() {
            let row = idx / (width * channels);
            let col = (idx % (width * channels)) / channels;
            let ch = (idx % (width * channels)) % channels;
            // Flip row to keep consistent orientation
            let flipped_row = height - row - 1;
            array[[flipped_row, col, ch]] = val;
        }

        array
    }

    fn width(&self) -> u32 {
        self.shape()[1] as u32
    }

    fn height(&self) -> u32 {
        self.shape()[0] as u32
    }
}
