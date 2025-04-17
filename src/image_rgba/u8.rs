use ndarray::Array3;
use png::{ColorType, Decoder, Encoder};
use std::{
    fmt::{Display, Formatter},
    fs::{File, create_dir_all},
    io::BufWriter,
    path::Path,
};

use crate::{ImageError, ImageRGBA};

impl ImageRGBA<u8> {
    /// Save the image in RGBA PNG format.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError> {
        let width = self.width() as u32;
        let height = self.height() as u32;
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        if let Some(parent) = path.as_ref().parent() {
            create_dir_all(parent).map_err(|err| {
                ImageError::from_message(format!(
                    "Failed to create directory {}: {}",
                    parent.display(),
                    err
                ))
            })?;
        }

        let file = File::create(&path).map_err(|err| {
            ImageError::from_message(format!(
                "Failed to create file {}: {}",
                path.as_ref().display(),
                err
            ))
        })?;
        let writer = BufWriter::new(file);
        let mut encoder = Encoder::new(writer, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG header: {}", err))
        })?;

        let data: Vec<_> = self.data.iter().copied().collect();
        writer.write_image_data(&data).map_err(|err| {
            ImageError::from_message(format!("Failed to write PNG data: {}", err))
        })?;
        Ok(())
    }

    /// Load a RGBA PNG image.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError> {
        let file = File::open(&path).map_err(|err| {
            ImageError::from_message(format!(
                "Failed to open file {}: {}",
                path.as_ref().display(),
                err
            ))
        })?;
        let decoder = Decoder::new(file);
        let mut reader = decoder
            .read_info()
            .map_err(|err| ImageError::from_message(format!("Failed to read PNG info: {}", err)))?;
        let mut buffer = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buffer).map_err(|err| {
            ImageError::from_message(format!("Failed to decode PNG frame: {}", err))
        })?;
        if info.color_type != ColorType::Rgba || info.bit_depth != png::BitDepth::Eight {
            return Err(ImageError::UnsupportedColorType);
        }

        let width = info.width as usize;
        let height = info.height as usize;
        let channels = 4;
        let total_bytes = width * height * channels;
        let data_vec: Vec<u8> = buffer[..total_bytes].to_vec();

        let data = Array3::from_shape_vec((height, width, channels), data_vec).map_err(|err| {
            ImageError::from_message(format!("Failed to create image array: {}", err))
        })?;
        Ok(Self { data })
    }

    /// Converts the image into a Vec of display lines.
    fn to_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.height());
        for row in self.data.outer_iter() {
            let mut line = String::new();
            for pixel in row.outer_iter() {
                let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                // Build a cell with background color based on pixel RGBA.
                use std::fmt::Write;
                write!(line, "\x1b[48;2;{r};{g};{b};{a}m  \x1b[0m").unwrap();
            }
            lines.push(line);
        }
        lines
    }

    /// Print a grid of ImageRGBA references in a 2D array.
    /// The grid width is determined by the terminal width divided by the image's printed width plus the gap.
    pub fn print_image_grid(images: &[&Self], gap: usize) -> Result<(), ImageError> {
        // Get terminal width (fallback to 80 columns if needed; using 60 as a placeholder here)
        let term_width = if let Some((w, _h)) = term_size::dimensions() {
            w
        } else {
            80 // Fallback width if terminal size cannot be determined
        };

        // Ensure there's at least one image.
        let first = images.first().ok_or_else(|| {
            ImageError::from_message("No images provided for grid display.".to_string())
        })?;
        let img_width = first.width(); // image width in pixels
        // Each pixel is printed using two characters ("  ")
        let cell_width = img_width * 2;

        // Calculate images per row considering the gap between each image.
        let images_per_row = if term_width < cell_width {
            1
        } else {
            (term_width + (2 * gap)) / (cell_width + (2 * gap))
        };

        // Process images by chunks (each chunk forms a row)
        for row in images.chunks(images_per_row) {
            // Convert each image in the current row to its lines.
            let lines_vec: Vec<Vec<String>> = row.iter().map(|img| img.to_lines()).collect();
            // Determine the row height (assumes all images have equal height, otherwise use the maximum).
            let row_height = lines_vec.iter().map(|lines| lines.len()).max().unwrap_or(0);
            // Print the row line by line.
            for line_idx in 0..row_height {
                for (i, lines) in lines_vec.iter().enumerate() {
                    // Print the gap before each tile except the first.
                    if i > 0 {
                        print!("{:width$}", "", width = gap * 2);
                    }
                    if line_idx < lines.len() {
                        print!("{}", lines[line_idx]);
                    } else {
                        // Fill in with spaces if an image has fewer lines.
                        print!("{:width$}", "", width = cell_width);
                    }
                }
                println!();
            }
            for _ in 0..gap {
                println!(); // Add a gap between rows.
            }
        }
        Ok(())
    }

    /// Print a grid of ImageRGBA references with captions in a 2D array.
    /// Each tuple contains a caption and an image reference.
    /// The grid width is determined by the terminal width divided by the image's printed width plus the gap.
    pub fn print_image_grid_with_caption(
        images_with_caption: &[(&Self, String)],
        gap: usize,
    ) -> Result<(), ImageError> {
        // Get terminal width (fallback to 80 columns if needed; using 60 as a placeholder here)
        let term_width = if let Some((w, _h)) = term_size::dimensions() {
            w
        } else {
            60 // Fallback width if terminal size cannot be determined
        };

        // Ensure there's at least one image.
        let first = images_with_caption.first().ok_or_else(|| {
            ImageError::from_message("No images provided for grid display.".to_string())
        })?;
        let img_width = first.0.width(); // image width in pixels
        // Each pixel prints as two characters ("  ")
        let cell_width = img_width * 2;

        // Calculate images per row considering the gap between each image.
        println!("term_width: {term_width}");
        let images_per_row = if term_width < cell_width {
            1
        } else {
            (term_width + (2 * gap)) / (cell_width + (2 * gap))
        };

        // Process each chunk (row) of images with captions.
        for row in images_with_caption.chunks(images_per_row) {
            // Convert each image in the current row into its display lines.
            let lines_vec: Vec<Vec<String>> = row.iter().map(|(img, _)| img.to_lines()).collect();
            // Find the maximum height in this row (assumes equal heights, else use the max).
            let row_height = lines_vec.iter().map(|lines| lines.len()).max().unwrap_or(0);

            // Print each line of the image grid.
            for line_idx in 0..row_height {
                for (i, lines) in lines_vec.iter().enumerate() {
                    if i > 0 {
                        print!("{:width$}", "", width = gap * 2);
                    }
                    if line_idx < lines.len() {
                        print!("{}", lines[line_idx]);
                    } else {
                        // Fill in with spaces if an image has fewer lines.
                        print!("{:width$}", "", width = cell_width);
                    }
                }
                println!();
            }

            // Print a single line with captions centered below each image.
            for (i, (_, caption)) in row.iter().enumerate() {
                if i > 0 {
                    print!("{:width$}", "", width = gap * 2);
                }
                // If the caption is longer than cell_width, trim it; otherwise, center align.
                let caption_print = if caption.len() > cell_width {
                    caption[..cell_width].to_string()
                } else {
                    format!("{:^width$}", caption, width = cell_width)
                };
                print!("{}", caption_print);
            }
            println!();

            // Add a vertical gap between rows.
            for _ in 0..gap {
                println!();
            }
        }
        Ok(())
    }
}

impl Display for ImageRGBA<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.data.outer_iter() {
            for pixel in row.outer_iter() {
                let r = pixel[0];
                let g = pixel[1];
                let b = pixel[2];
                let a = pixel[3];
                write!(f, "\x1b[48;2;{r};{g};{b};{a}m  \x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
