//! # Art
//!
//! A library for modeling artistic concepts. This module looks good but the API
//! documentation is not clear. We can improve it by re-exporting it

// Re-exporting 
pub use crate::art::kinds::PrimaryColor;
pub use crate::art::kinds::SecondaryColor;
pub use crate::art::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}