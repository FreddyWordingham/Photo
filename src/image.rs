use std::path::Path;

use crate::image_error::ImageError;

pub trait Image {
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ImageError>;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self, ImageError>
    where
        Self: Sized;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
