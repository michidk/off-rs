use off_rs::{
    geometry::mesh::Mesh,
    parser::{color_format::ColorFormat, options::Options},
    writer::{self, error::Error},
};

const CUBE: &str = include_str!("../examples/cube.off");
const WIKI: &str = include_str!("../benches/resources/wiki.off");
const PRINSTON: &str = include_str!("../benches/resources/prinston.off");

/// Parses `off`, writes the mesh and checks that parsing the output gives the same mesh.
fn assert_roundtrip(off: &str, color_format: ColorFormat) -> Mesh {
    let parser_options = Options {
        color_format,
        ..Default::default()
    };
    let writer_options = writer::options::Options { color_format };

    let mesh = off_rs::parse(off, parser_options).unwrap();
    let written = off_rs::to_off_string(&mesh, writer_options).unwrap();
    let reparsed = off_rs::parse(&written, parser_options).unwrap();

    assert_eq!(mesh, reparsed);
    mesh
}

#[test]
fn roundtrip_cube() {
    let mesh = assert_roundtrip(CUBE, ColorFormat::RGBAFloat);
    assert_eq!(mesh.vertex_count(), 8);
    assert_eq!(mesh.face_count(), 6);
}

#[test]
fn roundtrip_wiki() {
    assert_roundtrip(WIKI, ColorFormat::RGBInteger);
}

#[test]
fn roundtrip_prinston() {
    assert_roundtrip(PRINSTON, ColorFormat::RGBAFloat);
}

#[test]
fn write_to_writer_and_path() {
    let mesh = off_rs::parse(CUBE, Options::default()).unwrap();
    let expected = off_rs::to_off_string(&mesh, Default::default()).unwrap();

    let mut buffer = Vec::new();
    off_rs::write(&mesh, &mut buffer, Default::default()).unwrap();
    assert_eq!(String::from_utf8(buffer).unwrap(), expected);

    let path = std::env::temp_dir().join(format!("off-rs-write-{}.off", std::process::id()));
    off_rs::to_path(&mesh, &path, Default::default()).unwrap();
    let from_file = off_rs::from_path(&path, Options::default()).unwrap();
    std::fs::remove_file(&path).unwrap();

    assert_eq!(from_file, mesh);
}

#[test]
fn io_error_has_source() {
    use std::error::Error as _;

    let mesh = Mesh::default();
    let error =
        off_rs::to_path(&mesh, "/nonexistent-dir/mesh.off", Default::default()).unwrap_err();

    assert!(matches!(error, Error::Io(_)));
    assert!(error.source().is_some());
}
