use crate::colors::{EightBit, True, simple};
use crate::{Color, Colorizer, DisplayWithExact, DisplayWithFallback};

/// The main trait to allow a type to be colored.
pub trait Colorize: DisplayWithExact + DisplayWithFallback + Sized {
    /// Provides text coloring.
    fn color<C: Color>(self, color: C) -> Colorizer<C, Self> {
        Colorizer::foreground(self, color)
    }

    /// Provides background coloring.
    fn on_color<C: Color>(self, color: C) -> Colorizer<C, Self> {
        Colorizer::background(self, color)
    }

    /// Makes the text black.
    fn black(self) -> Colorizer<simple::Black, Self> {
        self.color(simple::Black)
    }

    /// Makes the text red.
    fn red(self) -> Colorizer<simple::Red, Self> {
        self.color(simple::Red)
    }

    /// Makes the text green.
    fn green(self) -> Colorizer<simple::Green, Self> {
        self.color(simple::Green)
    }

    /// Makes the text blue.
    fn blue(self) -> Colorizer<simple::Blue, Self> {
        self.color(simple::Blue)
    }

    /// Makes the text yellow.
    fn yellow(self) -> Colorizer<simple::Yellow, Self> {
        self.color(simple::Yellow)
    }

    /// Makes the text cyan.
    fn cyan(self) -> Colorizer<simple::Cyan, Self> {
        self.color(simple::Cyan)
    }

    /// Makes the text magenta.
    fn magenta(self) -> Colorizer<simple::Magenta, Self> {
        self.color(simple::Magenta)
    }

    /// Makes the text white.
    fn white(self) -> Colorizer<simple::White, Self> {
        self.color(simple::White)
    }

    /// Makes the background black.
    fn on_black(self) -> Colorizer<simple::Black, Self> {
        self.on_color(simple::Black)
    }

    /// Makes the background red.
    fn on_red(self) -> Colorizer<simple::Red, Self> {
        self.on_color(simple::Red)
    }

    /// Makes the background green.
    fn on_green(self) -> Colorizer<simple::Green, Self> {
        self.on_color(simple::Green)
    }

    /// Makes the background blue.
    fn on_blue(self) -> Colorizer<simple::Blue, Self> {
        self.on_color(simple::Blue)
    }

    /// Makes the background yellow.
    fn on_yellow(self) -> Colorizer<simple::Yellow, Self> {
        self.on_color(simple::Yellow)
    }

    /// Makes the background cyan.
    fn on_cyan(self) -> Colorizer<simple::Cyan, Self> {
        self.on_color(simple::Cyan)
    }

    /// Makes the background magenta.
    fn on_magenta(self) -> Colorizer<simple::Magenta, Self> {
        self.on_color(simple::Magenta)
    }

    /// Makes the background white.
    fn on_white(self) -> Colorizer<simple::White, Self> {
        self.on_color(simple::White)
    }

    /// Makes the text bright black.
    fn bright_black(self) -> Colorizer<simple::BrightBlack, Self> {
        self.color(simple::BrightBlack)
    }

    /// Makes the text bright red.
    fn bright_red(self) -> Colorizer<simple::BrightRed, Self> {
        self.color(simple::BrightRed)
    }

    /// Makes the text bright green.
    fn bright_green(self) -> Colorizer<simple::BrightGreen, Self> {
        self.color(simple::BrightGreen)
    }

    /// Makes the text bright blue.
    fn bright_blue(self) -> Colorizer<simple::BrightBlue, Self> {
        self.color(simple::BrightBlue)
    }

    /// Makes the text bright yellow.
    fn bright_yellow(self) -> Colorizer<simple::BrightYellow, Self> {
        self.color(simple::BrightYellow)
    }

    /// Makes the text bright cyan.
    fn bright_cyan(self) -> Colorizer<simple::BrightCyan, Self> {
        self.color(simple::BrightCyan)
    }

    /// Makes the text bright magenta.
    fn bright_magenta(self) -> Colorizer<simple::BrightMagenta, Self> {
        self.color(simple::BrightMagenta)
    }

    /// Makes the text bright white.
    fn bright_white(self) -> Colorizer<simple::BrightWhite, Self> {
        self.color(simple::BrightWhite)
    }

    /// Makes the background bright black.
    fn on_bright_black(self) -> Colorizer<simple::BrightBlack, Self> {
        self.on_color(simple::BrightBlack)
    }

    /// Makes the background bright red.
    fn on_bright_red(self) -> Colorizer<simple::BrightRed, Self> {
        self.on_color(simple::BrightRed)
    }

    /// Makes the background bright green.
    fn on_bright_green(self) -> Colorizer<simple::BrightGreen, Self> {
        self.on_color(simple::BrightGreen)
    }

    /// Makes the background bright blue.
    fn on_bright_blue(self) -> Colorizer<simple::BrightBlue, Self> {
        self.on_color(simple::BrightBlue)
    }

    /// Makes the background bright yellow.
    fn on_bright_yellow(self) -> Colorizer<simple::BrightYellow, Self> {
        self.on_color(simple::BrightYellow)
    }

    /// Makes the background bright cyan.
    fn on_bright_cyan(self) -> Colorizer<simple::BrightCyan, Self> {
        self.on_color(simple::BrightCyan)
    }

    /// Makes the background bright magenta.
    fn on_bright_magenta(self) -> Colorizer<simple::BrightMagenta, Self> {
        self.on_color(simple::BrightMagenta)
    }

    /// Makes the background bright white.
    fn on_bright_white(self) -> Colorizer<simple::BrightWhite, Self> {
        self.on_color(simple::BrightWhite)
    }

    /// Sets the text to a color according to the 8-bit lookup table.
    fn eight_bit(self, lookup: u8) -> Colorizer<EightBit, Self> {
        self.color(EightBit::from(lookup))
    }

    /// Sets the background to a color according to the 8-bit lookup table.
    fn on_eight_bit(self, lookup: u8) -> Colorizer<EightBit, Self> {
        self.on_color(EightBit::from(lookup))
    }

    /// Sets text to a true color.
    fn rgb(self, r: u8, g: u8, b: u8) -> Colorizer<True, Self> {
        self.color(True::from_rgb(r, g, b))
    }

    /// Sets background to a true color.
    fn on_rgb(self, r: u8, g: u8, b: u8) -> Colorizer<True, Self> {
        self.on_color(True::from_rgb(r, g, b))
    }
}

impl<D: DisplayWithExact + DisplayWithFallback> Colorize for D {}
