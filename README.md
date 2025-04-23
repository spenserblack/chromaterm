# `chromaterm`

[![Crates.io Version](https://img.shields.io/crates/v/chromaterm)](https://crates.io/crates/chromaterm)

Yet another crate for terminal colors.

[docs.rs](https://docs.rs/chromaterm/)

## Some goals

- Avoiding allocations caused by generating new types like `String`
- Recognizing that color support is a _spectrum_, not a _boolean_
- Providing granular controls over color support levels and how to handle them

## Basic Usage

```rust
use chromaterm::prelude::*;

// This crate is configurable, but requires some initialization.
chromaterm::config::use_default_color_support();
chromaterm::config::convert_to_supported(true);

// # Colorization
println!("Hello, {}!", "World".green().on_blue().bold());
```

Colorization is available on `&str` and all types that implement `Deref<Target=str>`
(like `String`).

You can run `cargo run --example basic`, too.

## Trade-offs

There are some limitations that you might run into, compared to other libraries.

### Types

The types are much more strict compared to some libraries like [colored][colored], due to the heavy use of
generics. `"".red().on_green()` is a different type than `"".on_green().red()`.

### No overwriting colors

In [colored][colored]'s implementation, `"red or green".red().green()` will set the text to the color
green. In this crate, `"red or green".red().green()` will be _red._ This is because
colorization and styling is implemented by _wrapping_ an inner type.

This isn't the exact implementation, but a handy visualization is that
`"red or green".red().green()` is equivalent to `Green(Red("red or green"))`.

[colored]: https://crates.io/crates/colored
