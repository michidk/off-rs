//! A simple benchmark which tries to parse a list of off documents.
//!
//! The results of the benchmark are **not** compared against other libraries
//! but instead are used to check/test against regressions.
//!
//! # Run
//!
//! To run the benchmark run the following in a terminal:
//!
//! ```bash
//! cargo bench parse
//! ```

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use off_rs::geometry::ColorFormat;
use off_rs::mesh::ParserOptions;
use off_rs::parser::Parser;

/// OFF file from wikipedia.
const WIKI_OFF: &'static str = include_str!("resources/wiki.off");

/// OFF file from the prinston off specification.
const PRINSTON_OFF: &'static str = include_str!("resources/prinston.off");

pub fn criterion_benchmark(c: &mut Criterion) {
    // Creates a new benchmark function for the wiki example
    c.bench_function("parse wiki - off-rs", |b| {
        let opts = ParserOptions {
            color_format: ColorFormat::RGBInteger,
            ..Default::default()
        };

        b.iter(|| black_box(Parser::new(&WIKI_OFF, opts).parse()))
    });

    // Creates a new benchmark function for the prinston example
    c.bench_function("parse prinston - off-rs", |b| {
        let opts = ParserOptions {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        };

        b.iter(|| black_box(Parser::new(&PRINSTON_OFF, opts).parse()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
