//! This crate provides utilities for coloring and styling strings.
//!
//! # Example
//!
//! ```rust,no_run
//! use chromaterm::prelude::*;
//!
//! // This configures the crate to automatically detect what colors are supported by
//! // the user's terminal. If the user's environment doesn't support the colors you
//! // use, it will automatically try to convert to a supported color value. Not all
//! // terminals support "true colors" (RGB), for example.
//! chromaterm::config::use_default_color_support();
//! chromaterm::config::convert_to_supported(true);
//!
//! println!("Hello, {}!", "World".green().italic().on_rgb(0, 64, 255));
//! ```
//!
//! ## Advanced usage
//!
//! ### Color support
//!
//! If you want to have more control over how color support is detected, use the
//! [`ColorSupport`] type.
//!
//! ```rust,no_run
//! use chromaterm::ColorSupport;
//!
//! // Detects support from the environment, and additionally respects the well-known
//! // NO_COLOR environment variable.
//! let support = ColorSupport::from_env().respect_no_color();
//! chromaterm::config::use_color_support(support);
//! ```
//!
//! ### Configuration
//!
//! ```rust
//! use chromaterm::prelude::*;
//! use chromaterm::ColorSupport;
//!
//! // Full color support ("true colors" are supported).
//! chromaterm::config::use_color_support(ColorSupport::True);
//! assert_eq!("styled".rgb(255, 0, 0).to_string(), "\x1B[38;2;255;0;0mstyled\x1B[0m");
//!
//! // Only the basic 16 colors are supported, and we try to convert to the closest
//! // color.
//! chromaterm::config::use_color_support(ColorSupport::Simple);
//! chromaterm::config::convert_to_supported(true);
//! assert_eq!("bright red".rgb(255, 0, 0).to_string(), "bright red".bright_red().to_string());
//!
//! // Now we refuse to convert to supported colors. If our color can't be displayed,
//! // we won't color at all! This can be useful if you feel that converting to less
//! // accurate colors can ruin the look of your output, and it's better to fall back
//! // to uncolored text.
//! chromaterm::config::convert_to_supported(false);
//! assert_eq!("not bright red".rgb(255, 0, 0).to_string(), "not bright red");
//!
//! // No colors are supported.
//! chromaterm::config::use_color_support(ColorSupport::None);
//!
//! // Because no colors are supported, the string is plain.
//! assert_eq!("not styled".rgb(255, 0, 0).to_string(), "not styled");
//! ```
pub use color::Color;
pub use color_level::ColorLevel;
pub use color_support::ColorSupport;
pub use colorize::Colorize;
pub use colorizer::Colorizer;
pub use colors::Colors;
pub use display::{DisplayWithExact, DisplayWithFallback};
pub use style::Style;
pub use styler::Styler;
pub use styles::Styles;
pub use stylize::Stylize;

mod color;
mod color_level;
mod color_support;
mod colorize;
mod colorizer;
pub mod colors;
pub mod config;
pub mod conversion;
mod display;
pub mod prelude;
mod style;
mod styler;
pub mod styles;
mod stylize;
