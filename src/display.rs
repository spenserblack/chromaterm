use crate::ColorSupport;
use std::fmt;
/// Like `Display`, but checks for color support. It should disable any coloring or
/// styling that isn't supported.
pub trait DisplayWithExact {
    /// Formats the value, checking for color support. If the color is not supported,
    /// the value is formatted without color.
    fn fmt_exact(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error>;

    /// Converts to a type that is suitable for display. Only shows the color if it is supported.
    fn display_exact(self, supported: ColorSupport) -> DisplayExact<Self>
    where
        Self: Sized,
    {
        DisplayExact {
            inner: self,
            support: supported,
        }
    }
}

/// Like `Display`, but checks for color support. If the coloring or styling isn't
/// supported, it should attempt to convert to coloring or styling that *is* supported.
pub trait DisplayWithFallback {
    /// Formats the value, checking for color support. If the color is not supported,
    /// it will try to convert to a color that is supported.
    fn fmt_fallback(
        &self,
        supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error>;

    /// Converts to a type that is suitable for display. Will try to convert to a color that is supported.
    fn display_fallback(self, supported: ColorSupport) -> DisplayFallback<Self>
    where
        Self: Sized,
    {
        DisplayFallback {
            inner: self,
            support: supported,
        }
    }
}

impl DisplayWithExact for str {
    /// Formats the value, never checking for color support, because it is assumed
    /// that a raw `str` does not need to check for color support.
    fn fmt_exact(
        &self,
        _supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl DisplayWithFallback for str {
    /// Formats the value, never checking for color support, because it is assumed
    /// that a raw `str` does not need to check for color support.
    fn fmt_fallback(
        &self,
        _supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl DisplayWithExact for &str {
    /// Formats the value, never checking for color support, because it is assumed
    /// that a raw `str` does not need to check for color support.
    fn fmt_exact(
        &self,
        _supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

impl DisplayWithFallback for &str {
    /// Formats the value, never checking for color support, because it is assumed
    /// that a raw `str` does not need to check for color support.
    fn fmt_fallback(
        &self,
        _supported: ColorSupport,
        f: &mut fmt::Formatter<'_>,
    ) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}

/// A type suitable for displaying values. Wraps a `DisplayWithSupport` with a
/// `ColorSupport`. Only shows the color if it is supported.
pub struct DisplayExact<D: DisplayWithExact> {
    inner: D,
    support: ColorSupport,
}

impl<D: DisplayWithExact> fmt::Display for DisplayExact<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt_exact(self.support, f)
    }
}

/// A type suitable for displaying values. Wraps a `DisplayWithSupport` with a
/// `ColorSupport`. Will try to convert to a color that is supported.
pub struct DisplayFallback<D: DisplayWithFallback> {
    inner: D,
    support: ColorSupport,
}

impl<D: DisplayWithFallback> fmt::Display for DisplayFallback<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.inner.fmt_fallback(self.support, f)
    }
}
