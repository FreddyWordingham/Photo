use chromatic::Colour;
use ndarray::Array2;
use num_traits::Float;
use std::{
    io::{Read, Write},
    path::Path,
};

use crate::PngError;

mod array2;

/// Trait for image encoding/decoding operations on `Array2<C>` where `C` is a type implementing the `Colour` trait.
pub trait Image<C, T, const N: usize>
where
    C: Colour<T, N> + Clone,
    T: Float + Send + Sync,
{
    /// Read an image from a file path.
    fn load<P: AsRef<Path>>(path: P) -> Result<Array2<C>, PngError>;

    /// Read an image from a reader.
    fn read<R: Read>(reader: R) -> Result<Array2<C>, PngError>;

    /// Write an image to a file path.
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), PngError>;

    /// Write an image to a writer.
    fn write<W: Write>(&self, writer: W) -> Result<(), PngError>;
}
