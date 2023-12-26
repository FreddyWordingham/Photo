//! Validation error structure.

use core::fmt::{Display, Formatter, Result};
use std::error::Error;

/// Validation error.
#[derive(Debug)]
pub struct ValidationError {
    /// Error message.
    message: String,
}

impl ValidationError {
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

impl Display for ValidationError {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Validation Error: {}", self.message)
    }
}

impl Error for ValidationError {}
