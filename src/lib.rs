//! # `Photo`
//!
//! `Photo` is a utility library for manipulating images in Rust.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(unreachable_code)]

mod channels;
mod image;

pub use channels::Channels;
pub use image::Image;
