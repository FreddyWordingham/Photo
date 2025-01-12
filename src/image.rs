use std::path::Path;

pub trait Image {
    fn save<P: AsRef<Path>>(&self, path: P);
    fn load<P: AsRef<Path>>(path: P) -> Self;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
