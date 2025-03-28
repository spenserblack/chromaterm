use crate::ColorLevel;
use std::fmt;

/// Marks a type as being able to represent a style.
///
/// # Example implementation
///
/// ```rust,no_run
/// # use chromaterm::Style;
/// use std::fmt;
///
/// struct Bold;
///
/// impl Style for Bold {
///     fn fmt_style(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
///         write!(f, "1")
///     }
/// }
/// ```
pub trait Style {
    /// Writes the ANSI text for text styling. Does not need to include the `\e[` at the
    /// start or the `m` at the end.
    fn fmt_style(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error>;
    /// Gets the color level. It's generally assumed that, if there's *any* color
    /// support, there's also style support.
    fn level(&self) -> ColorLevel {
        ColorLevel::Simple
    }
}
