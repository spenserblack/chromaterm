//! Get and store global configuration.
use crate::ColorSupport;
use std::sync::atomic::{AtomicU8, Ordering};

const ORDERING: Ordering = Ordering::Relaxed;

impl ColorSupport {
    const BIT_SHIFT: u8 = 6;

    const fn as_config(&self) -> u8 {
        let value = match self {
            Self::None => 0b00,
            Self::Simple => 0b01,
            Self::EightBit => 0b10,
            Self::True => 0b11,
        };
        value << Self::BIT_SHIFT
    }

    const fn from_config(config: u8) -> Self {
        const MASK: u8 = 0b11_000000;
        match (config & MASK) >> Self::BIT_SHIFT {
            0b00 => Self::None,
            0b01 => Self::Simple,
            0b10 => Self::EightBit,
            0b11 => Self::True,
            _ => unreachable!("Invalid bitwise logic"),
        }
    }

    const fn set_config(&self, config: u8) -> u8 {
        const MASK: u8 = 0b00_111111;
        let config = config & MASK;
        config | self.as_config()
    }
}

/// Should this crate attempt to fall back to simpler colors when the used colors
/// aren't supported?
#[derive(Debug)]
enum Fallback {
    /// Fall back to simpler colors.
    Yes,
    /// Don't show unsupported colors.
    No,
}

impl Fallback {
    const BIT_SHIFT: u8 = 5;

    const fn as_config(&self) -> u8 {
        let value: u8 = match self {
            Self::Yes => 1,
            Self::No => 0,
        };
        value << Self::BIT_SHIFT
    }

    const fn from_config(config: u8) -> Self {
        const MASK: u8 = 0b00_1_00000;
        match (config & MASK) >> Self::BIT_SHIFT {
            0 => Self::No,
            1 => Self::Yes,
            _ => unreachable!("Invalid bitwise logic"),
        }
    }

    const fn set_config(&self, config: u8) -> u8 {
        const MASK: u8 = 0b11_0_11111;
        let config = config & MASK;
        config | self.as_config()
    }
}

impl From<bool> for Fallback {
    #[inline]
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

impl From<Fallback> for bool {
    #[inline]
    fn from(value: Fallback) -> Self {
        match value {
            Fallback::Yes => true,
            Fallback::No => false,
        }
    }
}

const DEFAULT_COLOR_SUPPORT_CONFIG: u8 = ColorSupport::None.as_config();
const DEFAULT_CONVERT_TO_SUPPORTED_CONFIG: u8 = Fallback::Yes.as_config();
static CONFIG: AtomicU8 =
    AtomicU8::new(DEFAULT_COLOR_SUPPORT_CONFIG | DEFAULT_CONVERT_TO_SUPPORTED_CONFIG);

/// Stores the raw config.
fn store_config(value: u8) {
    CONFIG.store(value, ORDERING);
}

/// Gets the config value.
fn get_config() -> u8 {
    CONFIG.load(ORDERING)
}

/// Sets the global color support configuration.
pub fn use_color_support(support: ColorSupport) {
    let config = get_config();
    let config = support.set_config(config);
    store_config(config);
}

/// Gets the current globally-set color support configuration.
pub fn get_color_support() -> ColorSupport {
    let config = get_config();
    ColorSupport::from_config(config)
}

/// Use a reasonable default for color support.
pub fn use_default_color_support() {
    let support = ColorSupport::from_env().respect_no_color();
    use_color_support(support);
}

/// Returns if the crate will automatically try to convert colors to a supported color
/// value if you use a value that isn't by the user's environment.
pub fn get_convert_to_supported() -> bool {
    let config = get_config();
    Fallback::from_config(config).into()
}

/// Controls if this crate should automatically try to convert colors to supported
/// color values if you use a value that isn't supported by the user's environment.
pub fn convert_to_supported(should_convert: bool) {
    let config = get_config();
    let config = Fallback::from(should_convert).set_config(config);
    store_config(config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // NOTE This must all be in one test, as running in parallel can cause confusing
        //      behavior.
        store_config(0);
        use_color_support(ColorSupport::True);
        assert_eq!(
            CONFIG.load(ORDERING),
            0b11_0_00000,
            "color support should be set to True"
        );
        convert_to_supported(true);
        assert_eq!(
            CONFIG.load(ORDERING),
            0b11_1_00000,
            "convert to supported bit should be set"
        );
        convert_to_supported(false);
        assert_eq!(
            CONFIG.load(ORDERING),
            0b11_0_00000,
            "convert to supported bit should be unset"
        );
    }
}
