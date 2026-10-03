use off_rs::{
    Error,
    parser::{color_format::ColorFormat, options::Options},
};

#[test]
fn missing_vertex_color() {
    let off_string = r#"
OFF
3 1 0
-0.500000 -0.500000 0.500000 12 122 210
0.500000 -0.500000 0.500000 34 112
-0.500000 0.500000 0.500000 123 12 44
3 0 1 2
"#;

    let off = off_rs::parse(
        off_string,
        Options {
            color_format: ColorFormat::RGBInteger,
            ..Default::default()
        },
    );

    assert!(matches!(
        off.unwrap_err(),
        Error::ParserError(off_rs::parser::error::Error {
            kind: off_rs::parser::error::Kind::InvalidColor,
            line_index: 4,
            ..
        })
    ));
}

#[test]
fn error_chain_is_reachable() {
    use std::error::Error as _;

    let off = off_rs::parse("OFF\n1 0\n1.0 x 3.0\n", Options::default());
    let error = off.unwrap_err();

    // `off_rs::Error` -> `parser::error::Error` -> `ParseFloatError`
    let parser_error = error.source().expect("parser error");
    assert_eq!(parser_error.to_string(), "InvalidVertexPosition @ ln:3");

    let cause = parser_error.source().expect("underlying cause");
    assert_eq!(cause.to_string(), "invalid float literal");
}
