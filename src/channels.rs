//! ## `channels` image format.
//!
//! This module provides the `Channels` enum which represents the standard
//! image formats with varying numbers of channels: greyscale, greyscale with
//! alpha transparency, RGB (red, green, blue), and RGBA (RGB with alpha).

/// Channel formats supported by an `Image`.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channels {
    /// Greyscale.
    Grey = 1,
    /// Greyscale with alpha
    GreyAlpha = 2,
    /// Red, Green, Blue
    RGB = 3,
    /// Red, Green, Blue with alpha
    RGBA = 4,
}

impl Channels {
    /// Create `Channels` variant from the number of channels.
    #[inline]
    #[must_use]
    pub const fn from_num_channels(num: usize) -> Option<Self> {
        match num {
            1 => Some(Self::Grey),
            2 => Some(Self::GreyAlpha),
            3 => Some(Self::RGB),
            4 => Some(Self::RGBA),
            _ => None,
        }
    }

    /// Get the number of channels for this format.
    #[inline]
    #[must_use]
    pub const fn num_channels(&self) -> usize {
        match *self {
            Self::Grey => 1,
            Self::GreyAlpha => 2,
            Self::RGB => 3,
            Self::RGBA => 4,
        }
    }

    /// Check if this format has an alpha channel.
    #[inline]
    #[must_use]
    pub const fn has_alpha(&self) -> bool {
        matches!(self, &Self::GreyAlpha | &Self::RGBA)
    }

    /// Check if this format is greyscale (with or without alpha).
    #[inline]
    #[must_use]
    pub const fn is_greyscale(&self) -> bool {
        matches!(self, &Self::Grey | &Self::GreyAlpha)
    }

    /// Check if this format is color (RGB or RGBA).
    #[inline]
    #[must_use]
    pub const fn is_colour(&self) -> bool {
        matches!(self, &Self::RGB | &Self::RGBA)
    }
}
