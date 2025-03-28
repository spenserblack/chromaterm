use crate::ColorLevel;
use std::env;

/// Detected color support.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum ColorSupport {
    /// No color support.
    None,
    /// 8 or 16 bit color codes (3- or 4-bit).
    Simple,
    /// 256 color support.
    EightBit,
    /// True color support (24-bit).
    True,
}

impl ColorSupport {
    /// Calculates the color support from environment variables.
    pub fn from_env() -> Self {
        Self::from_term_env("COLORTERM")
            .or_else(|| Self::from_term_env("TERM"))
            .unwrap_or(Self::None)
    }

    /// Checks if the color level is supported.
    ///
    /// Not that this is *not* the same as a simple equality check.
    pub fn is_supported(&self, level: Self) -> bool {
        *self >= level
    }

    /// Respect the `NO_COLOR` environment variable.
    pub fn respect_no_color(self) -> Self {
        if env_to_bool("NO_COLOR") {
            Self::None
        } else {
            self
        }
    }

    /// Calculates from the named environment variable, which should be either `TERM`
    /// or `COLORTERM`.
    #[inline]
    fn from_term_env(key: &str) -> Option<Self> {
        debug_assert!(key == "TERM" || key == "COLORTERM");
        env::var(key).map(Self::from_term_value).unwrap_or(None)
    }

    /// Calculate from the `TERM` or `COLORTERM` environment variable.
    fn from_term_value<S: AsRef<str>>(value: S) -> Option<Self> {
        let value = value.as_ref();
        [
            ("256", Self::EightBit),
            ("24bit", Self::True),
            ("truecolor", Self::True),
        ]
        .into_iter()
        .find_map(|(hint, variant)| value.contains(hint).then_some(variant))
    }
}

impl From<ColorLevel> for ColorSupport {
    #[inline]
    fn from(value: ColorLevel) -> Self {
        match value {
            ColorLevel::Simple => Self::Simple,
            ColorLevel::EightBit => Self::EightBit,
            ColorLevel::True => Self::True,
        }
    }
}

impl From<Option<ColorLevel>> for ColorSupport {
    fn from(value: Option<ColorLevel>) -> Self {
        value.map(Self::from).unwrap_or(Self::None)
    }
}

/// Gets an environment variable as a boolean.
#[inline]
fn env_to_bool(key: &str) -> bool {
    env::var(key).map(env_value_to_bool).unwrap_or(false)
}

/// Converts an environment variable value to a boolean.
#[inline]
fn env_value_to_bool<S: AsRef<str>>(value: S) -> bool {
    matches!(value.as_ref(), "1" | "true")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("256-color", Some(ColorSupport::EightBit))]
    #[case("24bit", Some(ColorSupport::True))]
    #[case("truecolor", Some(ColorSupport::True))]
    fn test_from_term_value(#[case] value: &str, #[case] expected: Option<ColorSupport>) {
        assert_eq!(ColorSupport::from_term_value(value), expected);
    }

    #[rstest]
    #[case(ColorSupport::Simple, ColorSupport::Simple, true)]
    #[case(ColorSupport::EightBit, ColorSupport::Simple, true)]
    #[case(ColorSupport::True, ColorSupport::Simple, true)]
    #[case(ColorSupport::Simple, ColorSupport::EightBit, false)]
    #[case(ColorSupport::EightBit, ColorSupport::EightBit, true)]
    #[case(ColorSupport::True, ColorSupport::EightBit, true)]
    #[case(ColorSupport::Simple, ColorSupport::True, false)]
    #[case(ColorSupport::EightBit, ColorSupport::True, false)]
    #[case(ColorSupport::True, ColorSupport::True, true)]
    #[case(ColorSupport::None, ColorSupport::Simple, false)]
    #[case(ColorSupport::None, ColorSupport::EightBit, false)]
    #[case(ColorSupport::None, ColorSupport::True, false)]
    fn test_is_supported(
        #[case] detected: ColorSupport,
        #[case] level: ColorSupport,
        #[case] expected: bool,
    ) {
        assert_eq!(detected.is_supported(level), expected);
    }

    #[rstest]
    #[case("1", true)]
    #[case("true", true)]
    #[case("false", false)]
    #[case("0", false)]
    #[case("", false)]
    fn test_env_value_to_bool(#[case] value: &str, #[case] expected: bool) {
        assert_eq!(env_value_to_bool(value), expected);
    }
}
