use crate::{DisplayWithExact, DisplayWithFallback, Style, Styler, styles};

/// The main trait to allow a type to be styled.
pub trait Stylize: DisplayWithExact + DisplayWithFallback + Sized {
    /// Provides styling.
    fn style<S: Style>(self, style: S) -> Styler<S, Self> {
        Styler::new(self, style)
    }

    /// Makes the text bold.
    fn bold(self) -> Styler<styles::Bold, Self> {
        self.style(styles::Bold)
    }

    /// Makes the text dim.
    fn dim(self) -> Styler<styles::Dim, Self> {
        self.style(styles::Dim)
    }

    /// Italicizes the text.
    fn italic(self) -> Styler<styles::Italic, Self> {
        self.style(styles::Italic)
    }

    /// Underlines the text.
    fn underline(self) -> Styler<styles::Underline, Self> {
        self.style(styles::Underline)
    }

    /// Strikes through the text.
    fn strike(self) -> Styler<styles::Strike, Self> {
        self.style(styles::Strike)
    }
}

impl<D: DisplayWithExact + DisplayWithFallback> Stylize for D {}
