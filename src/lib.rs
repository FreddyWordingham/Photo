mod colour_map;
mod image;
mod image_error;
mod image_g;
mod image_ga;
mod image_rgb;
mod image_rgba;
mod norm_float;
mod transformation;

pub use colour_map::ColourMap;
pub use image::Image;
pub use image_error::ImageError;
pub use image_g::ImageG;
pub use image_ga::ImageGA;
pub use image_rgb::ImageRGB;
pub use image_rgba::ImageRGBA;
use norm_float::NormFloat;
pub use transformation::{ALL_TRANSFORMATIONS, Transformation};
