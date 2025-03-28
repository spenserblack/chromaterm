use crate::ColorSupport;

/// The color level.
#[derive(Debug, Copy, Clone)]
pub enum ColorLevel {
    /// 8 or 16 bit color codes (3- or 4-bit).
    Simple,
    /// 256 color support.
    EightBit,
    /// True color support (24-bit).
    True,
}

impl From<ColorSupport> for Option<ColorLevel> {
    #[inline]
    fn from(value: ColorSupport) -> Self {
        use ColorLevel::*;

        let level = match value {
            ColorSupport::None => return Self::None,
            ColorSupport::Simple => Simple,
            ColorSupport::EightBit => EightBit,
            ColorSupport::True => True,
        };
        Some(level)
    }
}
