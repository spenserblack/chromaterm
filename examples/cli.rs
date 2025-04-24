//! Example for CLI integration.
use chromaterm::ColorSupport;
use chromaterm::prelude::*;
use clap::{Parser, ValueEnum};

/// An example CLI for color integration.
#[derive(Parser)]
struct Cli {
    /// Specify the types of colors that should be used for display.
    #[arg(long, short, default_value = "auto")]
    color: ColorChoice,
}

/// Color choices for display.
#[derive(Clone, ValueEnum)]
enum ColorChoice {
    /// Automatically detect if colors are supported.
    Auto,
    /// Always use colors.
    Always,
    /// Limit to 8-bit colors and basic ANSI colors.
    EightBit,
    /// Limit to only basic ANSI colors.
    Ansi,
    /// Disable colors.
    Never,
}

impl ColorChoice {
    /// Sets up the color support level.
    fn setup(&self) {
        let support_level = match self {
            Self::Auto => {
                chromaterm::config::use_default_color_support();
                return;
            }
            Self::Always => ColorSupport::True,
            Self::EightBit => ColorSupport::EightBit,
            Self::Ansi => ColorSupport::Simple,
            Self::Never => ColorSupport::None,
        };
        chromaterm::config::use_color_support(support_level);
    }
}

fn main() {
    let cli = Cli::parse();
    cli.color.setup();
    chromaterm::config::convert_to_supported(true);

    println!("Let's print a gradient!");
    (0..=255).step_by(4).for_each(|c| {
        let r = 255;
        let g = c / 2;
        let b = 255 - c;
        print!("{}", "-".on_rgb(r, g, b));
    });
    println!();

    println!("Run this example again and pass `-h` to see how to change color support.")
}
