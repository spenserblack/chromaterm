//! Collection of color types.
use crate::conversion::{ToEightBit, ToSimple};
use crate::{Color, ColorLevel};
pub use eight_bit::EightBit;
pub use simple::Simple;
use std::fmt;
pub use true_color::True;

mod eight_bit;
pub mod simple;
mod true_color;

/// Unifies all color types.
pub enum Colors {
    Simple(Simple),
    EightBit(EightBit),
    True(True),
}

impl Colors {
    pub fn new_true_color(r: u8, g: u8, b: u8) -> Self {
        Self::True(True::from_rgb(r, g, b))
    }

    pub fn new_eight_bit(lookup: u8) -> Self {
        Self::EightBit(EightBit::from(lookup))
    }
}

macro_rules! impl_new_simple_variant {
    ($($fn_name:ident : $variant:ident),+ ,) => {
        impl Colors {
            $(
                pub fn $fn_name() -> Self {
                    Self::Simple(Simple::$variant)
               }
            )+
        }
    };
}

impl_new_simple_variant!(
    new_black:          Black,
    new_red:            Red,
    new_green:          Green,
    new_blue:           Blue,
    new_yellow:         Yellow,
    new_cyan:           Cyan,
    new_magenta:        Magenta,
    new_white:          White,
    new_bright_black:   BrightBlack,
    new_bright_red:     BrightRed,
    new_bright_green:   BrightGreen,
    new_bright_blue:    BrightBlue,
    new_bright_yellow:  BrightYellow,
    new_bright_cyan:    BrightCyan,
    new_bright_magenta: BrightMagenta,
    new_bright_white:   BrightWhite,
);

impl Color for Colors {
    fn fmt_fg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Simple(c) => c.fmt_fg(f),
            Self::EightBit(c) => c.fmt_fg(f),
            Self::True(c) => c.fmt_fg(f),
        }
    }

    fn fmt_bg(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Simple(c) => c.fmt_bg(f),
            Self::EightBit(c) => c.fmt_bg(f),
            Self::True(c) => c.fmt_bg(f),
        }
    }

    fn level(&self) -> ColorLevel {
        match self {
            Self::Simple(c) => c.level(),
            Self::EightBit(c) => c.level(),
            Self::True(c) => c.level(),
        }
    }

    fn rgb_u8(&self) -> (u8, u8, u8) {
        match self {
            Self::Simple(c) => c.rgb_u8(),
            Self::EightBit(c) => c.rgb_u8(),
            Self::True(c) => c.rgb_u8(),
        }
    }
}

impl From<Simple> for Colors {
    fn from(value: Simple) -> Self {
        Self::Simple(value)
    }
}

impl From<EightBit> for Colors {
    fn from(value: EightBit) -> Self {
        Self::EightBit(value)
    }
}

impl From<True> for Colors {
    fn from(value: True) -> Self {
        Self::True(value)
    }
}

impl ToSimple for Colors {
    fn to_simple(&self) -> Simple {
        match self {
            Colors::Simple(c) => *c,
            Colors::EightBit(c) => c.to_simple(),
            Colors::True(c) => c.to_simple(),
        }
    }
}

impl ToEightBit for Colors {
    fn to_eight_bit(&self) -> EightBit {
        match self {
            Colors::Simple(c) => c.to_eight_bit(),
            Colors::EightBit(c) => *c,
            Colors::True(c) => c.to_eight_bit(),
        }
    }
}
