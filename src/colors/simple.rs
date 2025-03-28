use super::EightBit;
use crate::conversion::{ToEightBit, ToSimple};
use crate::{Color, ColorLevel};
use std::fmt;

/// The black color.
pub struct Black;

/// The red color.
pub struct Red;

/// The green color.
pub struct Green;

/// The yellow color.
pub struct Yellow;

/// The blue color.
pub struct Blue;

/// The magenta color.
pub struct Magenta;

/// The cyan color.
pub struct Cyan;

/// The white color.
pub struct White;

/// the Bright Black color.
pub struct BrightBlack;

/// The Bright Red color.
pub struct BrightRed;

/// The Bright Green color.
pub struct BrightGreen;

/// The Bright Yellow color.
pub struct BrightYellow;

/// The Bright Blue color.
pub struct BrightBlue;

/// The Bright Magenta color.
pub struct BrightMagenta;

/// The Bright Cyan color.
pub struct BrightCyan;

/// The Bright White color.
pub struct BrightWhite;

macro_rules! impl_simple_color {
    ($c:ident, $fg:literal, $bg:literal, ($r:literal, $g:literal, $b:literal)) => {
        impl Color for $c {
            fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, $fg)
            }

            fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, $bg)
            }

            fn level(&self) -> ColorLevel {
                ColorLevel::Simple
            }

            fn rgb_u8(&self) -> (u8, u8, u8) {
                ($r, $g, $b)
            }
        }

        impl From<&$c> for Simple {
            fn from(_: &$c) -> Self {
                Self::$c
            }
        }

        impl From<$c> for Simple {
            fn from(_: $c) -> Self {
                Self::$c
            }
        }
    };
}

impl_simple_color!(Black, "30", "40", (0, 0, 0));
impl_simple_color!(Red, "31", "41", (0xAA, 0, 0));
impl_simple_color!(Green, "32", "42", (0, 0xAA, 0));
impl_simple_color!(Yellow, "33", "43", (0xAA, 0xAA, 0));
impl_simple_color!(Blue, "34", "44", (0, 0, 0xAA));
impl_simple_color!(Magenta, "35", "45", (0xAA, 0, 0xAA));
impl_simple_color!(Cyan, "36", "46", (0, 0xAA, 0xAA));
impl_simple_color!(White, "37", "47", (0xAA, 0xAA, 0xAA));
impl_simple_color!(BrightBlack, "90", "100", (0x55, 0x55, 0x55));
impl_simple_color!(BrightRed, "91", "101", (0xFF, 0x55, 0x55));
impl_simple_color!(BrightGreen, "92", "102", (0x55, 0xFF, 0x55));
impl_simple_color!(BrightYellow, "93", "103", (0xFF, 0xFF, 0x55));
impl_simple_color!(BrightBlue, "94", "104", (0x55, 0x55, 0xFF));
impl_simple_color!(BrightMagenta, "95", "105", (0xFF, 0x55, 0xFF));
impl_simple_color!(BrightCyan, "96", "106", (0x55, 0xFF, 0xFF));
impl_simple_color!(BrightWhite, "97", "107", (0xFF, 0xFF, 0xFF));

/// Unifies the simple colors.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Simple {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

macro_rules! impl_simple_enum {
    ($($c:ident),+) => {
        impl Color for Simple {
            fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                match self {
                    $(Self::$c => $c.fmt_fg(f)),+
                }
            }

            fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                match self {
                    $(Self::$c => $c.fmt_bg(f)),+
                }
            }

            fn level(&self) -> ColorLevel {
                match self {
                    $(Self::$c => $c.level()),+
                }
            }

            fn rgb_u8(&self) -> (u8, u8, u8) {
                match self {
                    $(Self::$c => $c.rgb_u8()),+
                }
            }
        }

        $(
            impl ToSimple for $c {
                fn to_simple(&self) -> Simple {
                    Simple::$c
                }
            }
        )+
    };
}

impl_simple_enum!(
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
);

impl Simple {
    /// All "dark" variants, ordered by blue, red, and green collected into a `u8`. For
    /// example, red would be `0b001`, green would be `0b010`, and cyan would be
    /// `0b100`. This allows for a simple way to generate an index that gets the
    /// approximate color.
    const DARK_BITWISE_INDEX: [Self; 8] = [
        Self::Black,
        Self::Red,
        Self::Green,
        Self::Yellow,
        Self::Blue,
        Self::Magenta,
        Self::Cyan,
        Self::White,
    ];

    /// All bright variants, ordered by blue, red, and green collected into a `u8`. For
    /// example, bright red would be `0b001`, bright green would be `0b010`, and bright
    /// cyan would be `0b100`. This allows for a simple way to generate an index that gets
    /// the approximate color.
    const BRIGHT_BITWISE_INDEX: [Self; 8] = [
        Self::BrightBlack,
        Self::BrightRed,
        Self::BrightGreen,
        Self::BrightYellow,
        Self::BrightBlue,
        Self::BrightMagenta,
        Self::BrightCyan,
        Self::BrightWhite,
    ];

    /// Tries to get the closes color from the RGB values.
    #[inline]
    pub const fn closest(r: u8, g: u8, b: u8) -> Self {
        // NOTE We assume a color is a bright variant if it's at least 0xC0.
        let is_bright = (r | g | b) >= 0xC0;
        let r = r >> 7;
        let g = g >> 7;
        let b = b >> 7;
        let index = (r | (g << 1) | (b << 2)) as usize;
        debug_assert!(index < Self::DARK_BITWISE_INDEX.len());
        debug_assert!(index < Self::BRIGHT_BITWISE_INDEX.len());
        if is_bright {
            Self::BRIGHT_BITWISE_INDEX[index]
        } else {
            Self::DARK_BITWISE_INDEX[index]
        }
    }
}

impl ToSimple for Simple {
    #[inline]
    fn to_simple(&self) -> Simple {
        *self
    }
}

macro_rules! impl_to_eight_bit {
    ($($n:literal : $c:ident),+) => {
        $(
            impl ToEightBit for $c {
                #[inline]
                fn to_eight_bit(&self) -> EightBit {
                    EightBit::from($n)
                }
            }
        )+

        impl ToEightBit for Simple {
            #[inline]
            fn to_eight_bit(&self) -> EightBit {
                match self {
                    $(Self::$c => EightBit::from($n)),+
                }
            }
        }
    };
}

impl_to_eight_bit! {
    0u8     : Black,
    1u8     : Red,
    2u8     : Green,
    3u8     : Yellow,
    4u8     : Blue,
    5u8     : Magenta,
    6u8     : Cyan,
    7u8     : White,
    8u8     : BrightBlack,
    9u8     : BrightRed,
    10u8    : BrightGreen,
    11u8    : BrightYellow,
    12u8    : BrightBlue,
    13u8    : BrightMagenta,
    14u8    : BrightCyan,
    15u8    : BrightWhite
}
