//! Parsing error structure.

use core::fmt::{Display, Formatter, Result};
use std::error::Error;

/// Parsing error.
#[derive(Debug)]
pub struct ParseError {
    /// Error message.
    message: String,
}

impl ParseError {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new(message: &str) -> Self {
        debug_assert!(!message.is_empty(), "Error message must not be empty!");

        Self {
            message: message.to_owned(),
        }
    }
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Parse Error: {}", self.message)
    }
}

impl Error for ParseError {}
