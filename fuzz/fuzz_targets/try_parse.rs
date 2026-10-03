//! A simple fuzz case which tries to parse random strings and checks if the
//! program (off-rs) panics or runs out of memory.
//!
//! # Run
//!
//! This will (for now) only work on linux machines.
//!
//! Requirements:
//!
//! ```bash
//! cargo install cargo-fuzz
//! ```
//!
//! To run the fuzz case enter the following in a terminal:
//!
//! ```bash
//! cargo fuzz run try_parse
//! ```

#![no_main]
use libfuzzer_sys::fuzz_target;
use off_rs::parser::Parser;
use off_rs::parser::color_format::ColorFormat;
use off_rs::parser::options::Options;

// Creates a new fuzz case which accepts random bytes as input.
fuzz_target!(|data: &[u8]| {
    // Tries to interpret the random bytes as string.
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Parser::new(
            &s,
            Options {
                color_format: ColorFormat::RGBAFloat,
                ..Default::default()
            },
        )
        .parse();
    }
});
