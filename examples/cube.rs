use std::path::Path;

use off_rs::{
    geometry::{color_format::ColorFormat, mesh::Mesh},
    parser::options::Options,
    FromPath,
};

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/cube.off");

fn main() {
    let mesh = Mesh::from_path(
        Path::new(PATH),
        Options {
            color_format: ColorFormat::RGBAFloat,
            ..Default::default()
        },
    );

    println!("{:#?}", mesh);
}
