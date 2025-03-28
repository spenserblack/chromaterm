use super::{EightBit, Simple};
use crate::conversion::{ToEightBit, ToSimple};
use crate::{Color, ColorLevel};
use std::error::Error;
use std::fmt;

/// A true color value.
pub struct True {
    r: u8,
    g: u8,
    b: u8,
}

impl True {
    /// Creates from RGB values.
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Tries to create from a hex value. The leading `#` is optional.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use chromaterm::colors::True;
    ///
    /// assert!(True::from_hex("#AABBCC").is_ok());
    /// assert!(True::from_hex("#aabbcc").is_ok());
    /// assert!(True::from_hex("aabbcc").is_ok());
    /// assert!(True::from_hex("abc").is_ok());
    /// assert!(True::from_hex("not a hex value").is_err());
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self, Box<dyn Error>> {
        let hex = if hex.starts_with('#') {
            hex.get(1..).unwrap()
        } else {
            hex
        };
        let len = hex.len();
        if len == 6 {
            Self::from_hex_long(hex)
        } else if len == 3 {
            Self::from_hex_short(hex)
        } else {
            Err("Invalid length".into())
        }
    }

    fn from_hex_short(hex: &str) -> Result<Self, Box<dyn Error>> {
        debug_assert_eq!(hex.len(), 3);
        let r = u8::from_str_radix(&hex[0..1], 16)?;
        let g = u8::from_str_radix(&hex[1..2], 16)?;
        let b = u8::from_str_radix(&hex[2..3], 16)?;
        Ok(Self::from_rgb(r | (r << 4), g | (g << 4), b | (b << 4)))
    }

    fn from_hex_long(hex: &str) -> Result<Self, Box<dyn Error>> {
        debug_assert_eq!(hex.len(), 6);
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Self::from_rgb(r, g, b))
    }
}

impl Color for True {
    fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "38;2;{};{};{}", self.r, self.g, self.b)
    }

    fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "48;2;{};{};{}", self.r, self.g, self.b)
    }

    fn level(&self) -> ColorLevel {
        ColorLevel::True
    }

    #[inline]
    fn rgb_u8(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl ToSimple for True {
    fn to_simple(&self) -> Simple {
        let (r, g, b) = self.rgb_u8();
        Simple::closest(r, g, b)
    }
}

impl ToEightBit for True {
    fn to_eight_bit(&self) -> EightBit {
        let (r, g, b) = self.rgb_u8();
        let r = full_color_to_cube(r);
        let g = full_color_to_cube(g);
        let b = full_color_to_cube(b);
        debug_assert!(r < 6 && g < 6 && b < 6);
        let lookup = 16 + (36 * r) + (6 * g) + b;
        EightBit::from(lookup)
    }
}

/// Reversed a full color in the range [0, 256) to a color channel for the eight-bit
/// 6x6x6 cube.
fn full_color_to_cube(channel: u8) -> u8 {
    if channel < 55 { 0 } else { (channel - 55) / 40 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::black(True::from_rgb(0, 0, 0), Simple::Black)]
    #[case::red(True::from_rgb(0xAA, 0, 0), Simple::Red)]
    #[case::yellow(True::from_rgb(0xAA, 0xAA, 0), Simple::Yellow)]
    #[case::green(True::from_rgb(0, 0xAA, 0), Simple::Green)]
    #[case::blue(True::from_rgb(0, 0, 0xAA), Simple::Blue)]
    #[case::white(True::from_rgb(0xAA, 0xAA, 0xAA), Simple::White)]
    #[case::bright_red(True::from_rgb(0xFF, 0, 0), Simple::BrightRed)]
    #[case::bright_yellow(True::from_rgb(0xFF, 0xFF, 0), Simple::BrightYellow)]
    #[case::bright_green(True::from_rgb(0, 0xFF, 0), Simple::BrightGreen)]
    #[case::bright_blue(True::from_rgb(0, 0, 0xFF), Simple::BrightBlue)]
    #[case::bright_white(True::from_rgb(0xFF, 0xFF, 0xFF), Simple::BrightWhite)]
    fn test_to_simple(#[case] true_color: True, #[case] expected: Simple) {
        assert_eq!(true_color.to_simple(), expected);
    }

    #[rstest]
    #[case(True::from_rgb(0, 0, 0), EightBit::from(16u8))]
    #[case(True::from_rgb(0xFF, 0xFF, 0xFF), EightBit::from(231u8))]
    #[case(True::from_rgb(0xFF, 0, 0), EightBit::from(196u8))]
    #[case(True::from_rgb(0, 0xFF, 0xFF), EightBit::from(51u8))]
    fn test_to_eight_bit(#[case] true_color: True, #[case] expected: EightBit) {
        assert_eq!(true_color.to_eight_bit(), expected);
    }
}
