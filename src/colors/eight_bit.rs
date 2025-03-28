use super::Simple;
use super::simple::*;
use crate::conversion::{ToEightBit, ToSimple};
use crate::{Color, ColorLevel};
use std::fmt;

/// An eight-bit color that uses a lookup table.
///
/// ```rust,no_run
/// # use chromaterm::colors::EightBit;
/// let color = EightBit::from(16u8);
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct EightBit(u8);

impl EightBit {
    /// Gets the RGB lookup color for the 6x6x6 cube.
    const fn cube_rgb_lookup(&self) -> (u8, u8, u8) {
        // TODO Simplify. Converting to the 6x6x6 and *then* converting to an eight-bit
        //      color channel is probably overly complicated.
        debug_assert!(16 <= self.0 && self.0 <= 231);
        let value = self.0 - 16;
        // NOTE Values with intensity 0 <= n < 6
        let r = (value / 36) % 6;
        let g = (value / 6) % 6;
        let b = value % 6;
        (
            Self::cube_to_intensity(r),
            Self::cube_to_intensity(g),
            Self::cube_to_intensity(b),
        )
    }

    /// Gets the RGB lookup for the grayscale values.
    const fn grayscale_rgb_lookup(&self) -> (u8, u8, u8) {
        debug_assert!(232 <= self.0);
        let value = self.0 - 232;
        let level = (value * 10) + 8;
        (level, level, level)
    }

    /// Converts an intensity value in the range [0, 6) to an eight-bit value.
    #[inline]
    const fn cube_to_intensity(value: u8) -> u8 {
        debug_assert!(value < 6);
        if value == 0 { 0 } else { (value * 40) + 55 }
    }
}

impl From<u8> for EightBit {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Color for EightBit {
    fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "38;5;{}", self.0)
    }

    fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "48;5;{}", self.0)
    }

    fn level(&self) -> ColorLevel {
        ColorLevel::EightBit
    }

    fn rgb_u8(&self) -> (u8, u8, u8) {
        match self.0 {
            0 => Black.rgb_u8(),
            1 => Red.rgb_u8(),
            2 => Green.rgb_u8(),
            3 => Yellow.rgb_u8(),
            4 => Blue.rgb_u8(),
            5 => Magenta.rgb_u8(),
            6 => Cyan.rgb_u8(),
            7 => White.rgb_u8(),
            8 => BrightBlack.rgb_u8(),
            9 => BrightRed.rgb_u8(),
            10 => BrightGreen.rgb_u8(),
            11 => BrightYellow.rgb_u8(),
            12 => BrightBlue.rgb_u8(),
            13 => BrightMagenta.rgb_u8(),
            14 => BrightCyan.rgb_u8(),
            15 => BrightWhite.rgb_u8(),
            16..=231 => self.cube_rgb_lookup(),
            232..=255 => self.grayscale_rgb_lookup(),
        }
    }
}

impl ToSimple for EightBit {
    fn to_simple(&self) -> Simple {
        match self.0 {
            0 => Simple::Black,
            1 => Simple::Red,
            2 => Simple::Green,
            3 => Simple::Yellow,
            4 => Simple::Blue,
            5 => Simple::Magenta,
            6 => Simple::Cyan,
            7 => Simple::White,
            8 => Simple::BrightBlack,
            9 => Simple::BrightRed,
            10 => Simple::BrightGreen,
            11 => Simple::BrightYellow,
            12 => Simple::BrightBlue,
            13 => Simple::BrightMagenta,
            14 => Simple::BrightCyan,
            15 => Simple::BrightWhite,
            16..=255 => {
                let (r, g, b) = self.rgb_u8();
                Simple::closest(r, g, b)
            }
        }
    }
}

impl ToEightBit for EightBit {
    #[inline]
    fn to_eight_bit(&self) -> EightBit {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(16, 0, 0, 0)]
    #[case(231, 0xFF, 0xFF, 0xFF)]
    #[case(21, 0, 0, 0xFF)]
    #[case(46, 0, 0xFF, 0)]
    #[case(196, 0xFF, 0, 0)]
    fn test_cube_rgb_lookup(
        #[case] lookup: u8,
        #[case] expected_r: u8,
        #[case] expected_g: u8,
        #[case] expected_b: u8,
    ) {
        let rgb = EightBit::from(lookup).cube_rgb_lookup();
        assert_eq!(rgb, (expected_r, expected_g, expected_b));
    }
}
