//! Saving error structure.

use core::fmt::{Display, Formatter, Result};
use std::error::Error;

/// Saving error.
#[derive(Debug)]
pub struct SaveError {
    /// Error message.
    message: String,
}

impl SaveError {
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

impl Display for SaveError {
    #[inline]
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "Save Error: {}", self.message)
    }
}

impl Error for SaveError {}
