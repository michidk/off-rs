#![no_main]
use libfuzzer_sys::fuzz_target;
use off_rs::document::ParserOptions;
use off_rs::geometry::ColorFormat;
use off_rs::parser::DocumentParser;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = DocumentParser::new(
            &s,
            ParserOptions {
                color_format: ColorFormat::RGBAFloat,
            },
        )
        .parse();
    }
});
