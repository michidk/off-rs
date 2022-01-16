#[allow(unused_imports)]
use off_rs::{document::*, geometry::*, parser::error::*, parser::*};

#[test]
fn missing_vertex_color() {
    let content = r#"
OFF
3 1 0
-0.500000 -0.500000 0.500000 12 122 210
0.500000 -0.500000 0.500000 34 112
-0.500000 0.500000 0.500000 123 12 44
3 0 1 2
"#;

    let off = OffDocument::parse(
        content,
        ParserOptions {
            color_format: ColorFormat::RGBInteger,
            ..Default::default()
        },
    );

    assert!(matches!(
        off.unwrap_err(),
        DocumentError::ParserError(ParserError {
            kind: ParserErrorKind::InvalidColor,
            line_index: 4,
            message: _
        })
    ));
}
