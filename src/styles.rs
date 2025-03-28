//! Text styles.
use crate::Style;
use std::fmt;

/// The bold style.
pub struct Bold;

/// The dim style.
pub struct Dim;

/// The italic style.
pub struct Italic;

/// The underline style.
pub struct Underline;

/// The strike style.
pub struct Strike;

macro_rules! impl_style {
    ($( ( $name:ident, $value:literal ) ),+) => {

        $(
            impl Style for $name {
                fn fmt_style(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                    write!(f, $value)
                }
            }
        )+

        /// Unifies styles into one value.
        pub enum Styles {
            $($name),+
        }

        impl Style for Styles {
            fn fmt_style(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                match self {
                    $(Self::$name => $name.fmt_style(f)),+
                }
            }
        }

        $(
            impl From<&$name> for Styles {
                fn from(_: &$name) -> Self {
                    Self::$name
                }
            }

            impl From<$name> for Styles {
                fn from(_: $name) -> Self {
                    Self::$name
                }
            }
        )+
    };
}

impl_style!(
    (Bold, "1"),
    (Dim, "2"),
    (Italic, "3"),
    (Underline, "4"),
    (Strike, "9")
);
