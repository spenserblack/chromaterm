use crate::{ColorSupport, DisplayWithExact, DisplayWithFallback, Style};
use std::fmt;

/// Wraps a value that can be colorized.
pub struct Styler<S: Style, D> {
    style: S,
    value: D,
}

impl<S: Style, D> Styler<S, D> {
    /// Wraps the displayable value with a style.
    pub fn new(value: D, style: S) -> Self {
        Self { style, value }
    }

    /// Displays the contained value, including the style if there's *any* color
    /// support.
    fn fmt_impl<F: FnOnce(&mut fmt::Formatter<'_>) -> Result<(), fmt::Error>>(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
        wrapped: F,
    ) -> Result<(), fmt::Error> {
        let level = self.style.level();
        let level: ColorSupport = level.into();
        let is_supported = supported >= level;
        if is_supported {
            write!(f, "\x1B[")?;
            self.style.fmt_style(f)?;
            write!(f, "m")?;
        }
        // self.value.fmt_exact(supported, f)?;
        wrapped(f)?;
        if is_supported {
            write!(f, "\x1B[0m")?;
        }
        Ok(())
    }
}

impl<S: Style, D: DisplayWithExact> DisplayWithExact for Styler<S, D> {
    /// Displays the contained value, including the style if there's *any* color
    /// support.
    #[inline]
    fn fmt_exact(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        self.fmt_impl(supported, f, |f| self.value.fmt_exact(supported, f))
    }
}

impl<S: Style, D: DisplayWithFallback> DisplayWithFallback for Styler<S, D> {
    /// Displays the contained value, including the style if there's *any* color
    /// support.
    #[inline]
    fn fmt_fallback(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        self.fmt_impl(supported, f, |f| self.value.fmt_fallback(supported, f))
    }
}

impl<S: Style, D: DisplayWithExact + DisplayWithFallback> fmt::Display for Styler<S, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // TODO Allow fallback vs exact to be configurable
        let support = crate::config::get_color_support();
        self.fmt_fallback(support, f)
    }
}
