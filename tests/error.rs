use off_rs::geometry::color_format::ColorFormat;
#[allow(unused_imports)]
use off_rs::{geometry::*, mesh::*, parser::*};

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

    let off = Mesh::parse(
        content,
        ParserOptions {
            color_format: ColorFormat::RGBInteger,
            ..Default::default()
        },
    );

    assert!(matches!(
        off.unwrap_err(),
        Error::ParserError(off_rs::parser::error::Error {
            kind: off_rs::parser::error::Kind::InvalidColor,
            line_index: 4,
            message: _
        })
    ));
}
