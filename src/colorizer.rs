use crate::{Color, ColorLevel, ColorSupport, DisplayWithExact, DisplayWithFallback, conversion};
use std::fmt;

/// Wraps a value that can be colorized.
pub struct Colorizer<C: Color, D> {
    color: ColorZone<C>,
    value: D,
}

impl<C: Color, D> Colorizer<C, D> {
    /// Wraps the displayable value with a foreground color.
    pub fn foreground(value: D, color: C) -> Self {
        Self {
            color: ColorZone::Fg(color),
            value,
        }
    }

    /// Wraps the displayable value with a background color.
    pub fn background(value: D, color: C) -> Self {
        Self {
            color: ColorZone::Bg(color),
            value,
        }
    }
}

impl<C: Color, D: DisplayWithExact> DisplayWithExact for Colorizer<C, D> {
    /// Displays the contained value, including the color *if it is supported.*
    fn fmt_exact(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        let level = self.color.level();
        let level: ColorSupport = level.into();
        let is_supported = supported >= level;
        if is_supported {
            self.color.fmt(f)?;
        }
        self.value.fmt_exact(supported, f)?;
        if is_supported {
            write!(f, "\x1B[0m")?;
        }
        Ok(())
    }
}

impl<C: Color + conversion::ToSimple + conversion::ToEightBit, D: DisplayWithFallback>
    DisplayWithFallback for Colorizer<C, D>
{
    /// Displays the contained value, and tries to fall back to a color that is supported.
    fn fmt_fallback(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        let level = self.color.level();
        match (supported, level) {
            (ColorSupport::None, _) => self.value.fmt_fallback(supported, f),
            // NOTE Since True is the highest level, we can avoid conversion for all branches.
            (ColorSupport::True, _)
            | (ColorSupport::EightBit, ColorLevel::EightBit)
            | (ColorSupport::EightBit, ColorLevel::Simple)
            | (ColorSupport::Simple, ColorLevel::Simple) => {
                self.color.fmt(f)?;
                self.value.fmt_fallback(supported, f)?;
                write!(f, "\x1B[0m")?;
                Ok(())
            }
            (ColorSupport::Simple, ColorLevel::EightBit)
            | (ColorSupport::Simple, ColorLevel::True) => {
                let color = self.color.map_borrow(|c| c.to_simple());
                color.fmt(f)?;
                self.value.fmt_fallback(supported, f)?;
                write!(f, "\x1B[0m")
            }
            (ColorSupport::EightBit, ColorLevel::True) => {
                let color = self.color.map_borrow(|c| c.to_eight_bit());
                color.fmt(f)?;
                self.value.fmt_fallback(supported, f)?;
                write!(f, "\x1B[0m")
            }
        }
    }
}

impl<
    C: Color + conversion::ToSimple + conversion::ToEightBit,
    D: DisplayWithExact + DisplayWithFallback,
> fmt::Display for Colorizer<C, D>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let support = crate::config::get_color_support();
        let should_fallback = crate::config::get_convert_to_supported();

        if should_fallback {
            self.fmt_fallback(support, f)
        } else {
            self.fmt_exact(support, f)
        }
    }
}

/// Whether the color is in the foreground or the background.
enum ColorZone<C: Color> {
    Fg(C),
    Bg(C),
}

impl<C: Color> ColorZone<C> {
    #[inline]
    fn level(&self) -> ColorLevel {
        let color = match self {
            Self::Fg(color) => color,
            Self::Bg(color) => color,
        };
        color.level()
    }

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "\x1B[")?;
        match self {
            Self::Fg(color) => {
                color.fmt_fg(f)?;
            }
            Self::Bg(color) => {
                color.fmt_bg(f)?;
            }
        }
        write!(f, "m")
    }

    /// Helper to map one color zone to another color zone without consuming the
    /// original.
    fn map_borrow<T: Color, F: FnOnce(&C) -> T>(&self, f: F) -> ColorZone<T> {
        match self {
            Self::Fg(c) => ColorZone::Fg(f(c)),
            Self::Bg(c) => ColorZone::Bg(f(c)),
        }
    }
}
