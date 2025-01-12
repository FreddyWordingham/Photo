use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use ndarray::Array2;
use num_traits::{Float, FromPrimitive};
use png::{ColorType, Decoder, Encoder};

use crate::image::Image;

impl<T> Image for Array2<T>
where
    T: Float + FromPrimitive,
{
    fn save<P: AsRef<Path>>(&self, path: P) {
        // Assert all values are in the range [0.0, 1.0]
        assert!(self.iter().all(|&x| x >= T::zero() && x <= T::one()));

        let path_ref = path.as_ref();

        let file = File::create(path_ref).unwrap();
        let ref mut writer = BufWriter::new(file);

        let width = self.width();
        let height = self.height();

        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        // Flip the rows vertically and collect into a Vec<u8>
        let data = self
            .outer_iter()
            .rev() // Reverse the rows
            .flat_map(|row| {
                row.iter()
                    .map(|&x| (x * T::from(255.0).unwrap()).to_u8().unwrap()) // Convert to u8
                    .collect::<Vec<u8>>() // Collect each row into a Vec<u8>
            })
            .collect::<Vec<u8>>(); // Collect the flattened data into a single Vec<u8>

        writer.write_image_data(&data).unwrap();
    }

    fn load<P: AsRef<Path>>(path: P) -> Self {
        let path_ref = path.as_ref();
        let file = File::open(path_ref).unwrap();

        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();

        // Ensure the image is grayscale
        assert_eq!(info.color_type, ColorType::Grayscale);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);

        let width = info.width as usize;
        let height = info.height as usize;

        // Create an Array2 and populate it with normalized pixel values
        let data = buf[..info.buffer_size()]
            .iter()
            .map(|&x| T::from_u8(x).unwrap() / T::from_u8(255).unwrap())
            .collect::<Vec<T>>();

        Array2::from_shape_vec((height, width), data).unwrap()
    }

    fn width(&self) -> u32 {
        self.ncols() as u32
    }

    fn height(&self) -> u32 {
        self.nrows() as u32
    }
}
