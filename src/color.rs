use crate::ColorLevel;
use std::fmt;

/// Marks a type as being able to represent a color.
///
/// # Example implementation
///
/// ```rust,no_run
/// # use chromaterm::{Color, ColorLevel};
/// use std::fmt;
///
/// struct Red;
///
/// impl Color for Red {
///     fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
///         write!(f, "31")
///     }
///
///     fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
///         write!(f, "41")
///     }
///
///     fn level(&self) -> ColorLevel {
///         ColorLevel::Simple
///     }
///
///     fn rgb_u8(&self) -> (u8, u8, u8) {
///         (0xFF, 0, 0)
///     }
/// }
/// ```
pub trait Color {
    /// Writes the ANSI text for foreground colorization. Does not need to include the
    /// `\e[` at the start or the `m` at the end.
    fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
    /// Writes the ANSI text for background colorization. Does not need to include the
    /// `\e[` at the start or the `m` at the end.
    fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
    /// Gets the color level. This helps prevent attempting to print a color when it
    /// isn't supported.
    fn level(&self) -> ColorLevel;
    /// Gets the RGB values as `u8`.
    fn rgb_u8(&self) -> (u8, u8, u8);
    /// Gets the RGB values as `u16`.
    fn rgb_u16(&self) -> (u16, u16, u16) {
        rgb_u8_to_u16(self.rgb_u8())
    }
    /// Gets the RGB values as `u32`.
    fn rgb_u32(&self) -> (u32, u32, u32) {
        rgb_u16_to_u32(self.rgb_u16())
    }
}

macro_rules! make_upcase {
    ($name:ident, $low:ident, $high:ident, $shift:literal) => {
        #[inline]
        const fn $name(low: $low) -> $high {
            let low = low as $high;
            (low << $shift) | low
        }
    };
}

make_upcase!(upcase_u8_to_u16, u8, u16, 8);
make_upcase!(upcase_u16_to_u32, u16, u32, 16);

macro_rules! tuple_upcase {
    ($name:ident, $upcaser:ident, $low:ident, $high:ident) => {
        #[inline]
        const fn $name((r, g, b): ($low, $low, $low)) -> ($high, $high, $high) {
            ($upcaser(r), $upcaser(g), $upcaser(b))
        }
    };
}

tuple_upcase!(rgb_u8_to_u16, upcase_u8_to_u16, u8, u16);
tuple_upcase!(rgb_u16_to_u32, upcase_u16_to_u32, u16, u32);
