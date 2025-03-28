//! Tools for converting to colors. Conversions may lose color precision.
//!
//! When implementing these, it should be somewhat safe to use `unreachable!()` or
//! similar panicking macros for *upwards* conversion, as this library only converts
//! to lower support levels.
use crate::colors::{EightBit, Simple};

/// Trait to mark that a value can be converted to simple colors.
pub trait ToSimple {
    /// Converts to a simple color.
    fn to_simple(&self) -> Simple;
}

/// Trait to mark that a value can be converted to eight-bit colors.
pub trait ToEightBit {
    /// Converts to an eight-bit color.
    fn to_eight_bit(&self) -> EightBit;
}
